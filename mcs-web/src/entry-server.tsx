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
        <body class="dark:bg-dark-bg dark:text-dark-fg">
          <div id="app" >{children}</div>
          <footer class="mt-2">
            <ul class="flex justify-center content-center italic gap-x-1">
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
