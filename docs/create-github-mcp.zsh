#!/usr/bin/env zsh
set -euo pipefail
setopt extended_glob

SCRIPT_DIR=${0:A:h}
TABLE_MD=${GITHUB_SPECS_MD:-"$SCRIPT_DIR/github-api-specs.md"}
PROJECT_DIR=${GITHUB_MCP_DIR:-"/Users/lucianoguerche/Documents/GitHub/github-mcp"}
SPEC_CACHE_DIR=${GITHUB_SPEC_CACHE_DIR:-"/tmp/github-mcp-specs"}
LANGUAGE=${MCPIFY_LANGUAGE:-"rust"}
DEFAULT_LABEL="gh-2026-03-10"
FORCE="${GITHUB_FORCE:-1}"

while (( $# > 0 )); do
  case "$1" in
    --force|-f)
      FORCE="1"
      shift
      ;;
    --help|-h)
      print "Usage: $0 [--force]"
      print ""
      print "Environment overrides:"
      print "  GITHUB_SPECS_MD       Path to github-api-specs.md"
      print "  GITHUB_MCP_DIR        Output project directory"
      print "  GITHUB_SPEC_CACHE_DIR Downloaded spec cache directory"
      print "  MCPIFY_LANGUAGE       mcpify output language, default: rust"
      print "  GITHUB_FORCE=0        Refuse to generate into a non-empty output directory"
      exit 0
      ;;
    *)
      print -u2 "Unknown argument: $1"
      print -u2 "Usage: $0 [--force]"
      exit 1
      ;;
  esac
done

if [[ ! -f "$TABLE_MD" ]]; then
  print -u2 "Spec table not found: $TABLE_MD"
  exit 1
fi

if [[ -d "$PROJECT_DIR" && -n "$(ls -A "$PROJECT_DIR" 2>/dev/null)" && "$FORCE" != "1" ]]; then
  print -u2 "Target directory already exists and is not empty: $PROJECT_DIR"
  print -u2 "Run with --force, or export GITHUB_FORCE=1, to pass --force to mcpify generation."
  exit 1
fi

for dependency in curl node mcpify; do
  if ! command -v "$dependency" >/dev/null 2>&1; then
    print -u2 "Required command not found: $dependency"
    exit 1
  fi
done

raw_github_url() {
  local url="$1"
  local path

  if [[ "$url" == https://github.com/github/rest-api-description/blob/main/* ]]; then
    path="${url#https://github.com/github/rest-api-description/blob/main/}"
    print -r -- "https://raw.githubusercontent.com/github/rest-api-description/main/$path"
  else
    print -r -- "$url"
  fi
}

download_spec() {
  local label="$1"
  local spec_url="$2"
  local raw_url extension raw_file normalized_file

  raw_url="$(raw_github_url "$spec_url")"
  extension="${raw_url:t:e}"
  [[ -z "$extension" ]] && extension="yaml"

  mkdir -p "$SPEC_CACHE_DIR"
  raw_file="$SPEC_CACHE_DIR/${label}.${extension}"
  normalized_file="$SPEC_CACHE_DIR/${label}.normalized.${extension}"

  print "Downloading $label -> $raw_file" >&2
  curl -fsSL -A "Mozilla/5.0" "$raw_url" -o "$raw_file"

  print "Normalizing GitHub auth schemes for $label -> $normalized_file" >&2
  normalize_spec "$raw_file" "$normalized_file"

  print -r -- "$normalized_file"
}

normalize_spec() {
  local input_file="$1"
  local output_file="$2"

  node - "$input_file" "$output_file" <<'NODE'
const fs = require("node:fs");

const inputFile = process.argv[2];
const outputFile = process.argv[3];
const original = fs.readFileSync(inputFile, "utf8");

const securitySchemes = {
  GitHubBearerToken: {
    type: "http",
    scheme: "bearer",
    bearerFormat: "PAT",
    description: "GitHub.com, GitHub Enterprise Cloud, and GitHub Enterprise Server REST API authentication using Authorization: Bearer with a personal access token, GitHub App token, OAuth app token, or GITHUB_TOKEN.",
  },
  GitHubTokenHeader: {
    type: "apiKey",
    in: "header",
    name: "Authorization",
    description: "GitHub REST API legacy token authentication using an Authorization header value such as token YOUR-TOKEN.",
  },
  GitHubBasicAuth: {
    type: "http",
    scheme: "basic",
    description: "GitHub REST API Basic authentication for specific GitHub App/OAuth app endpoints that require client ID and client secret, and for GitHub Enterprise Server username/password Basic auth where enabled.",
  },
};

const securitySchemeNames = Object.keys(securitySchemes);

function normalizeJson(text) {
  const spec = JSON.parse(text);
  spec.components ??= {};
  spec.components.securitySchemes ??= {};
  for (const [name, scheme] of Object.entries(securitySchemes)) {
    spec.components.securitySchemes[name] ??= scheme;
  }
  if (!Array.isArray(spec.security) || spec.security.length === 0) {
    spec.security = securitySchemeNames.map((name) => ({ [name]: [] }));
  }
  return `${JSON.stringify(spec, null, 2)}\n`;
}

function countLeadingSpaces(line) {
  return line.match(/^\s*/)[0].length;
}

function hasTopLevelKey(lines, key) {
  const pattern = new RegExp(`^${key}:\\s*(?:$|#)`);
  return lines.some((line) => pattern.test(line));
}

function hasSecuritySchemes(lines) {
  return lines.some((line) => /^\s*securitySchemes\s*:/.test(line));
}

function hasSecurityScheme(lines, name) {
  return lines.some((line) => new RegExp(`^\\s*${name}\\s*:`).test(line));
}

function securitySchemeBlock(name, indent) {
  const scheme = securitySchemes[name];
  const childIndent = " ".repeat(indent + 2);
  const propIndent = " ".repeat(indent + 4);
  const block = [`${childIndent}${name}:`];

  for (const [key, value] of Object.entries(scheme)) {
    block.push(`${propIndent}${key}: ${formatYamlScalar(value)}`);
  }

  return block;
}

function formatYamlScalar(value) {
  if (typeof value !== "string") return String(value);
  return JSON.stringify(value);
}

function missingSecuritySchemeBlocks(lines, indent) {
  return securitySchemeNames.flatMap((name) => {
    if (hasSecurityScheme(lines, name)) return [];
    return securitySchemeBlock(name, indent);
  });
}

function insertIntoExistingSecuritySchemes(lines) {
  const idx = lines.findIndex((line) => /^\s*securitySchemes\s*:/.test(line));
  if (idx < 0) return lines;

  return [
    ...lines.slice(0, idx + 1),
    ...missingSecuritySchemeBlocks(lines, countLeadingSpaces(lines[idx])),
    ...lines.slice(idx + 1),
  ];
}

function insertSecuritySchemesIntoComponents(lines) {
  const idx = lines.findIndex((line) => /^components\s*:/.test(line));
  if (idx < 0 || hasSecuritySchemes(lines)) return lines;

  return [
    ...lines.slice(0, idx + 1),
    "  securitySchemes:",
    ...missingSecuritySchemeBlocks(lines, 2),
    ...lines.slice(idx + 1),
  ];
}

function appendComponentsWithSecuritySchemes(lines) {
  if (hasTopLevelKey(lines, "components")) return lines;

  return [
    ...lines,
    "components:",
    "  securitySchemes:",
    ...missingSecuritySchemeBlocks(lines, 2),
  ];
}

function normalizeYaml(text) {
  let lines = text.replace(/\s+$/u, "").split(/\r?\n/);

  if (hasSecuritySchemes(lines)) {
    lines = insertIntoExistingSecuritySchemes(lines);
  } else if (hasTopLevelKey(lines, "components")) {
    lines = insertSecuritySchemesIntoComponents(lines);
  } else {
    lines = appendComponentsWithSecuritySchemes(lines);
  }

  if (!hasTopLevelKey(lines, "security")) {
    lines.push("security:", ...securitySchemeNames.map((name) => `  - ${name}: []`));
  }

  return `${lines.join("\n")}\n`;
}

let normalized;
try {
  normalized = normalizeJson(original);
} catch {
  normalized = normalizeYaml(original);
}

fs.writeFileSync(outputFile, normalized);
NODE
}

typeset -a rows
typeset -a skipped_versions
while IFS=$'\t' read -r version spec_url; do
  [[ -z "$version" || -z "$spec_url" ]] && continue
  rows+=("${version}"$'\t'"${spec_url}")
done < <(
  awk -F'|' '
    /^## Failed\/Skipped Versions/ { exit }
    /^\|/ && $2 !~ /Version/ && $2 !~ /^---/ {
      version=$2; url=$3;
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", version);
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", url);
      gsub(/^<|>$/, "", url);
      if (url ~ /^https?:\/\//) {
        print version "\t" url;
      }
    }
  ' "$TABLE_MD"
)

if (( ${#rows[@]} == 0 )); then
  print -u2 "No spec rows found in: $TABLE_MD"
  exit 1
fi

default_spec_url=""
for row in "${rows[@]}"; do
  IFS=$'\t' read -r version spec_url <<< "$row"
  if [[ "$version" == "$DEFAULT_LABEL" ]]; then
    default_spec_url="$spec_url"
    break
  fi
done

if [[ -z "$default_spec_url" ]]; then
  print -u2 "Default spec not found: $DEFAULT_LABEL"
  exit 1
fi

force_args=()
if [[ "$FORCE" == "1" ]]; then
  force_args=(--force)
fi

print "Generating GitHub MCP project in: $PROJECT_DIR"
print "Default OpenAPI spec: $DEFAULT_LABEL -> $default_spec_url"
default_spec_file="$(download_spec "$DEFAULT_LABEL" "$default_spec_url")"

set +e
mcpify \
  -i "$default_spec_file" \
  -o "$PROJECT_DIR" \
  --language "$LANGUAGE" \
  --api-version "$DEFAULT_LABEL" \
  "${force_args[@]}"
mcpify_status=$?
set -e
if (( mcpify_status != 0 )); then
  print -u2 ""
  print -u2 "Error: mcpify failed while generating the default version ($DEFAULT_LABEL), before any additional versions could be added."
  print -u2 "The add-version loop only runs after the default project and its mcp_store.db are created successfully."
  print -u2 "Last generated spec file: $default_spec_file"
  exit "$mcpify_status"
fi

for row in "${rows[@]}"; do
  IFS=$'\t' read -r version spec_url <<< "$row"

  if [[ "$version" == "$DEFAULT_LABEL" ]]; then
    continue
  fi

  if ! spec_file="$(download_spec "$version" "$spec_url")"; then
    print -u2 "Warning: skipping $version because the spec could not be downloaded: $spec_url"
    skipped_versions+=("$version download-failed $spec_url")
    continue
  fi

  print "Adding $version -> $spec_file"
  if ! mcpify add-version \
    --project "$PROJECT_DIR" \
    --version "$version" \
    -i "$spec_file"; then
    print -u2 "Warning: skipping $version because mcpify could not ingest this spec: $spec_file"
    skipped_versions+=("$version mcpify-add-version-failed $spec_url")
    continue
  fi
done

if (( ${#skipped_versions[@]} > 0 )); then
  print ""
  print "Skipped ${#skipped_versions[@]} version(s):"
  for skipped in "${skipped_versions[@]}"; do
    print "  - $skipped"
  done
fi

print "Done. GitHub MCP project created at: $PROJECT_DIR"
