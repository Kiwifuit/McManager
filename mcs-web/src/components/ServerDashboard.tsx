export default function DashboardNavBar(props: {
  displayName: string;
  id: string;
}) {
  return (
    <aside class="bg-light-accent dark:bg-dark-accent">
      <ul class="mt-2 flex h-[calc(100vh-3.75rem)] flex-col rounded-xl text-left">
        <p class="truncate px-4 py-2 text-2xl font-bold">{props.displayName}</p>
        <NavbarButton url={`/server/${props.id}/`} name="Server" />
        <NavbarButton url={`/server/${props.id}/options`} name="Options" />
        <NavbarButton url={`/server/${props.id}/console`} name="Console" />
        <NavbarButton url={`/server/${props.id}/logs`} name="Logs" />
        <NavbarButton url={`/server/${props.id}/players`} name="Players" />
        <NavbarButton url={`/server/${props.id}/worlds`} name="Worlds" />
        <NavbarButton url={`/server/${props.id}/software`} name="Software" />
        <NavbarButton url={`/server/${props.id}/modpacks`} name="Modpacks" />
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
