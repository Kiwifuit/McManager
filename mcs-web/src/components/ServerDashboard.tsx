export default function DashboardNavBar() {
  return (
    <aside class="bg-server-offline">
      <ul class="flex flex-col gap-3 text-center mt-2">
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
  )
}

function NavbarButton(props: {url: string, name: string}) {
  return (
    <li>
      <a class="bg-server-online px-3 py-2 rounded-sm w-full block shadow-dark-bg active:shadow-inner" href={props.url}>{props.name}</a>
    </li>
   )
}