export default function ServerListItem(props: {
  id: string,
  online: boolean,
  display_name: string,
  description: string,
  players: {
    active: number,
    total: number
  },
  software: string,
  modpack: string | null,
}) {
  return (
    <div class='dark:bg-dark-server-background bg-light-server-background mb-3 p-2 rounded-md'>
      <a href={`/server/${props.id}/`} class="grid grid-cols-2">
        <p class='text-xl ml-3'>{props.display_name}</p>
        <p class='ml-6 dark:text-dark-server-description text-light-server-description col-start-1 line-clamp-2 row-span-2'>{props.description ? props.description : (<i>No description</i>)}</p>
        {
          props.online ?
            (
              <p class="col-start-2 row-start-1 text-right mr-3">Status:
                <span class="text-server-online mx-1">●</span>
                <span>Online</span>
              </p>
            ) :
            (
              <p class="col-start-2 row-start-1 text-right mr-3">Status:
                <span class="text-server-offline mx-1">●</span>
                <span>Offline</span>
              </p>
            )
        }
        <p class="align-middle text-right mr-3">Players: {props.players.active} / {props.players.total}</p>
        <p class="align-middle col-start-2 text-right mr-3">Server: {props.software}</p>
      </a>
    </div>
  )
}