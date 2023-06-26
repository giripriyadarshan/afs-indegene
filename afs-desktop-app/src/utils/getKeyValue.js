export default function getKeyValue(status) {
    let kvStatus = []
    let errorStatus = []
    if (status.length > 0) {
        status.forEach((item, _) => {
          // check if item starts with "dev"
          if (item.startsWith('DEV') || item.startsWith('START')) {
            console.log(item)
            return
          }

          let itemArr = item.split(' | ');

          if (itemArr[0] === "END") {
            let x = {
              key: itemArr[2],
              value: itemArr[1]
            }

            kvStatus = [x]
            
            // break the loop
            return false
          }

          if (itemArr[0] === "ALL") {
            itemArr[0] = itemArr[2]
          }

          if (itemArr[1] === "FAILED") {
            let x = {
              key: itemArr[0],
              value: itemArr[1]
            }

            let found = errorStatus.find(element => element.key === itemArr[0])
            if (found) {
              let index = errorStatus.findIndex(element => element.key === itemArr[0])
              errorStatus[index] = x
            }
            else {
            errorStatus.push(x)
            }
          }
          
          let found = kvStatus.find(element => element.key === itemArr[0])
          if (found) {
            let index = kvStatus.findIndex(element => element.key === itemArr[0])
            kvStatus[index] = {
              key: itemArr[0],
              value: itemArr[1]
            }
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
      return {
        status: kvStatus,
        errors: errorStatus
      }
}