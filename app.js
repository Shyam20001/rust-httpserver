const githubRawUrl = "https://raw.githubusercontent.com/Shyam20001/rust-httpserver/main/index.html";

async function fetchContent() {
  const response = await fetch(githubRawUrl);
  if (!response.ok) {
    throw new Error(`Failed to fetch content: ${response.statusText}`); 
  }
  return await response.text();
}

Bun.serve({
  async fetch(req) {
    try {
      const htmlContent = await fetchContent();
      return new Response(htmlContent, {
        headers: {
          "Content-Type": "text/html",
        },
      });
    } catch (error) {
      return new Response(`Error: ${error.message}`, { status: 500 });
    }
  },
  port: process.env.PORT || 3000,
});

console.log(`Server running at http://localhost:${Bun.port || 3000}/`);