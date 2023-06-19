import './StatusList.css'

export default function StatusList(props) {
    return (
        <div className="status-container">
          <ul>
            {props.status.map((item, index) => 
                <li key={index}><span className={
                    item.value == "SUCCESS"? "success" : item.value == "PENDING"? "pending" : "failed"
                }></span> 
                {item.key} <i>----------</i> {item.value}
                </li>
            )}
          </ul>
        </div>
    )
}