// @refresh reload
import { createHandler, StartServer } from "@solidjs/start/server";
import { BsGithub } from 'solid-icons/bs';
import { FaSolidCode } from 'solid-icons/fa';

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
          <footer class="mt-2 text-x">
            <ul class="flex justify-center content-center italic gap-x-1">
              <li>MCS by Kiwifuit</li>
              <li class="flex items-center ml-1" >
                <a href="https://github.com/Kiwifuit" target="_blank" rel="noopener noreferrer">
                  <BsGithub />
                </a>
              </li>
              <li class="flex items-center ml-1 top-10" >
                <a href="https://github.com/Kiwifuit/McManager" target="_blank" rel="noopener noreferrer">
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
