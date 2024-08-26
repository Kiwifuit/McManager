// @refresh reload
import { createHandler, StartServer } from "@solidjs/start/server";

export default createHandler(() => (
  <StartServer
    document={({ assets, children, scripts }) => (
      <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <link rel="icon" href="/favicon.ico" />
          {assets}
        </head>
        <body>
          <div id="app">{children}</div>
          <footer>
            <ul>
              <li>MCS by Kiwifuit</li>
              <li><a href="https://github.com/Kiwifuit" target="_blank" rel="noopener noreferrer">Github</a></li>
              <li><a href="https://github.com/Kiwifuit/McManager" target="_blank" rel="noopener noreferrer">Repository</a></li>
            </ul>
          </footer>
          {scripts}
        </body>
      </html>
    )}
  />
));
