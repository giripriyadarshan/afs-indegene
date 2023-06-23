import './StatusList.css'

export default function StatusList(props) {
    return (
        <div className="status-container">
          <p className='table-title'>{props.title}</p>
          <table>
            <thead>
              <tr>
                <th>Status</th>
                <th>Keymessage</th>
              </tr>
            </thead>
            <tbody>
              {props.status.map((item, index) =>
                <tr key={index}>
                  <td className={
                    item.value == "SUCCESS"? "success" : item.value == "PENDING"? "pending" : "failed"
                  }></td>
                  <td>{item.key}</td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
    )
}

/*
convert the above into a table

<table>
  <thead>
    <tr>
      <th>Status</th>
      <th>Keymessage</th>
    </tr>
  </thead>
  <tbody>
    {props.status.map((item, index) =>
      <tr key={index}>
        <td className={
          item.value == "SUCCESS"? "success" : item.value == "PENDING"? "pending" : "failed"
        }></td>
        <td>{item.key}</td>
      </tr>
    )}
  </tbody>
</table>


*/