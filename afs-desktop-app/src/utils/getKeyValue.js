export default function getKeyValue(status) {
    let kvStatus = []
    if (status.length > 0) {
        status.forEach((item, _) => {
          let itemArr = item.split(' | ');
          
            let found = kvStatus.find(element => element.key == itemArr[0])
            if (found) {
              let index = kvStatus.findIndex(element => element.key == itemArr[0])
              let x = {
                key: itemArr[0],
                value: itemArr[1]
              }
              kvStatus[index] = x
            }
            else {
              let x = {
                key: itemArr[0],
                value: itemArr[1]
              }
              kvStatus.push(x)
            }  
        })
      }
      return kvStatus
}