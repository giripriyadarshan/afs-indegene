export default function getKeyValue(status) {
    let kvStatus = []
    if (status.length > 0) {
        status.forEach((item, _) => {
          // check if item starts with "dev"
          if (item.startsWith('DEV') || item.startsWith('START')) {
            console.log(item)
            return
          }

          let itemArr = item.split(' | ');

          if (itemArr[0] == "END") {
            let x = {
              key: itemArr[2],
              value: itemArr[1]
            }

            kvStatus = [x]
            
            // break the loop
            return false
          }

          if (itemArr[0] == "ALL") {
            itemArr[0] = itemArr[2]
          }
          
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