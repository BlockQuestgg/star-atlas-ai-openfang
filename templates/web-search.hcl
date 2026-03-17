# Earl Template — Web Search
# Performs a web search query and returns results as JSON.
# Used by the researcher agent for market research and industry monitoring.
#
# Usage: earl call --yes --json http.web-search --query "esports venues Iowa"

version    = 1
provider   = "http"
categories = ["web", "research"]

command "web-search" {
  title       = "Web search"
  summary     = "Search the web for a query"
  description = "Performs a web search and returns results. Used for market research, competitor analysis, and industry monitoring."

  annotations {
    mode    = "read"
    secrets = []
  }

  param "query" {
    type        = "string"
    required    = true
    description = "Search query string"
  }

  param "count" {
    type        = "number"
    required    = false
    default     = 10
    description = "Number of results to return"
  }

  operation {
    protocol = "http"
    method   = "GET"
    url      = "https://html.duckduckgo.com/html/"

    query = {
      q = "{{ args.query }}"
    }

    headers = {
      User-Agent = "Earl/1.0 (research agent)"
      Accept     = "text/html"
    }
  }

  result {
    decode = "text"
    output = "Search results for: {{ args.query }}"
  }
}
