export default function DashboardNavBar(props: { name: string }) {
  return (
    <aside class="bg-light-accent dark:bg-dark-accent">
      <ul class="mt-2 flex h-[calc(100vh-3.75rem)] flex-col rounded-xl text-left">
        <p class="truncate px-4 py-2 text-2xl font-bold">{props.name}</p>
        <NavbarButton url="./server" name="Server" />
        <NavbarButton url="./options" name="Options" />
        <NavbarButton url="./console" name="Console" />
        <NavbarButton url="./logs" name="Logs" />
        <NavbarButton url="./players" name="Players" />
        <NavbarButton url="./worlds" name="Worlds" />
        <NavbarButton url="./software" name="Software" />
        <NavbarButton url="./modpacks" name="Modpacks" />
      </ul>
    </aside>
  );
}

function NavbarButton(props: { url: string; name: string }) {
  return (
    <li>
      <a
        class="dark:hover:bg-dark-dashboard-hover hover:bg-light-dashboard-hover block w-full rounded-sm bg-light-accent px-3 py-2 shadow-dark-bg active:shadow-inner dark:bg-dark-accent"
        href={props.url}
      >
        {props.name}
      </a>
    </li>
  );
}
