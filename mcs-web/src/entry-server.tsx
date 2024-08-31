// @refresh reload
import { createHandler, StartServer } from "@solidjs/start/server";
import { BsGithub } from "solid-icons/bs";
import { FaSolidCode } from "solid-icons/fa";

export default createHandler(() => (
  <StartServer
    document={({ assets, children, scripts }) => (
      <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />

          <link rel="icon" href="/favicon.ico" />
          <link
            rel="apple-touch-icon"
            sizes="180x180"
            href="/apple-touch-icon.png"
          />
          <link
            rel="icon"
            type="image/png"
            sizes="32x32"
            href="/favicon-32x32.png"
          />
          <link
            rel="icon"
            type="image/png"
            sizes="16x16"
            href="/favicon-16x16.png"
          />
          <link rel="manifest" href="/site.webmanifest" />

          {assets}
        </head>
        <body class="bg-light-bg text-light-fg dark:bg-dark-bg dark:text-dark-fg">
          <div id="app">{children}</div>
          <footer class="text-x mt-2">
            <ul class="flex content-center justify-center gap-x-1 italic">
              <li>MCS by Kiwifuit</li>
              <li class="ml-1 flex items-center">
                <a
                  href="https://github.com/Kiwifuit"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <BsGithub />
                </a>
              </li>
              <li class="top-10 ml-1 flex items-center">
                <a
                  href="https://github.com/Kiwifuit/McManager"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <FaSolidCode />
                </a>
              </li>
            </ul>
          </footer>
          {scripts}
        </body>
      </html>
    )}
  />
));
