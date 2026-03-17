# Earl Template — Web Fetch
# Fetches a web page and returns its content.
# Used by the researcher agent to read articles, reports, and pages.
#
# Usage: earl call --yes --json http.web-fetch --url "https://example.com/article"

version    = 1
provider   = "http"
categories = ["web", "research"]

command "web-fetch" {
  title       = "Fetch web page"
  summary     = "Retrieve the content of a web page"
  description = "Fetches a URL and returns the page content. Used for reading articles, reports, and competitor pages."

  annotations {
    mode    = "read"
    secrets = []
  }

  param "url" {
    type        = "string"
    required    = true
    description = "URL to fetch"
  }

  operation {
    protocol = "http"
    method   = "GET"
    url      = "{{ args.url }}"

    headers = {
      User-Agent = "Earl/1.0 (research agent)"
      Accept     = "text/html, application/json, text/plain"
    }
  }

  result {
    decode = "text"
    output = "Fetched {{ args.url }}"
  }
}
