# GitHub REST API OpenAPI Spec Files

Extracted from <https://github.com/github/rest-api-description> on 2026-07-07. The default version for generation is `gh-2026-03-10`.

Total rows: 29

## Counts

```text
GitHub.com: 1
GitHub Enterprise Cloud: 1
GitHub Enterprise Server: 27
```

## Accepted Auth Schemes

GitHub's REST OpenAPI description files do not currently declare `components.securitySchemes`, so the generation script normalizes each downloaded spec before passing it to `mcpify`.

For the generated MCP server, normalize specs with these shared schemes across GitHub.com, GitHub Enterprise Cloud, and GitHub Enterprise Server:

```yaml
components:
  securitySchemes:
    GitHubBearerToken:
      type: http
      scheme: bearer
      bearerFormat: PAT
      description: GitHub.com, GitHub Enterprise Cloud, and GitHub Enterprise Server REST API authentication using Authorization: Bearer with a personal access token, GitHub App token, OAuth app token, or GITHUB_TOKEN.
    GitHubTokenHeader:
      type: apiKey
      in: header
      name: Authorization
      description: GitHub REST API legacy token authentication using an Authorization header value such as token YOUR-TOKEN.
    GitHubBasicAuth:
      type: http
      scheme: basic
      description: GitHub REST API Basic authentication for specific GitHub App/OAuth app endpoints that require client ID and client secret, and for GitHub Enterprise Server username/password Basic auth where enabled.
security:
  - GitHubBearerToken: []
  - GitHubTokenHeader: []
  - GitHubBasicAuth: []
```

| API surface | Versions | Accepted REST API auth schemes |
|---|---|---|
| GitHub.com | `gh-*` | Token in `Authorization` header using `Bearer` or `token`; fine-grained personal access token; personal access token (classic); GitHub App user or installation access token; OAuth app token; GitHub Actions `GITHUB_TOKEN`; Basic auth with app client ID and client secret for specific GitHub App/OAuth app endpoints. Username/password authentication is not supported. |
| GitHub Enterprise Cloud | `ghec-*` | Token in `Authorization` header using `Bearer` or `token`; fine-grained personal access token; personal access token (classic); GitHub App user or installation access token; OAuth app token; GitHub Actions `GITHUB_TOKEN`; Basic auth with app client ID and client secret for specific GitHub App/OAuth app endpoints. Username/password authentication is not supported. |
| GitHub Enterprise Server | `ghes-*` | Token in `Authorization` header using `Bearer` or `token`; personal access token; GitHub App user or installation access token; OAuth app token; GitHub Actions `GITHUB_TOKEN`; Basic auth with app client ID and client secret for specific GitHub App/OAuth app endpoints; username/password Basic auth is documented for GHES, although token authentication is recommended. |

Sources: [GitHub.com REST authentication](https://docs.github.com/en/rest/authentication/authenticating-to-the-rest-api), [GitHub Enterprise Cloud REST authentication](https://docs.github.com/en/enterprise-cloud@latest/rest/authentication/authenticating-to-the-rest-api), [GitHub Enterprise Server REST authentication](https://docs.github.com/en/enterprise-server@3.21/rest/authentication/authenticating-to-the-rest-api).

| Version | YAML File url |
|---|---|
| gh-2026-03-10 | <https://github.com/github/rest-api-description/blob/main/descriptions/api.github.com/api.github.com.2026-03-10.yaml> |
| ghec-2026-03-10 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghec/ghec.2026-03-10.yaml> |
| ghes-3.21 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.21/ghes-3.21.yaml> |
| ghes-3.20 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.20/ghes-3.20.yaml> |
| ghes-3.19 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.19/ghes-3.19.yaml> |
| ghes-3.18 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.18/ghes-3.18.yaml> |
| ghes-3.17 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.17/ghes-3.17.yaml> |
| ghes-3.16 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.16/ghes-3.16.yaml> |
| ghes-3.15 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.15/ghes-3.15.yaml> |
| ghes-3.14 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.14/ghes-3.14.yaml> |
| ghes-3.13 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.13/ghes-3.13.yaml> |
| ghes-3.12 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.12/ghes-3.12.yaml> |
| ghes-3.11 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.11/ghes-3.11.yaml> |
| ghes-3.10 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.10/ghes-3.10.yaml> |
| ghes-3.9 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.9/ghes-3.9.yaml> |
| ghes-3.8 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.8/ghes-3.8.yaml> |
| ghes-3.7 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.7/ghes-3.7.yaml> |
| ghes-3.6 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.6/ghes-3.6.yaml> |
| ghes-3.5 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.5/ghes-3.5.yaml> |
| ghes-3.4 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.4/ghes-3.4.yaml> |
| ghes-3.3 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.3/ghes-3.3.yaml> |
| ghes-3.2 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.2/ghes-3.2.yaml> |
| ghes-3.1 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.1/ghes-3.1.yaml> |
| ghes-3.0 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-3.0/ghes-3.0.yaml> |
| ghes-2.22 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-2.22/ghes-2.22.yaml> |
| ghes-2.21 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-2.21/ghes-2.21.yaml> |
| ghes-2.20 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-2.20/ghes-2.20.yaml> |
| ghes-2.19 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-2.19/ghes-2.19.yaml> |
| ghes-2.18 | <https://github.com/github/rest-api-description/blob/main/descriptions/ghes-2.18/ghes-2.18.yaml> |
