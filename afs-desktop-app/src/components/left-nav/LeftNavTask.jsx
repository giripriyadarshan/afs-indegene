import classes from './LeftNavTask.module.css';
import Veeva from '../../assets/leftNavIcons/veeva-icon-black.svg'
import More from '../../assets/leftNavIcons/more-horizontal.svg'


/* 
    Deploys task
    tasks are defined in a key value pair below
        the key is the name of the task
        the value is the icon which will be displayed
    the task is deployed on click and the event is bubbled up to the parent component
*/
export default function LeftNavTask(props) {
    const tasks = {
        veeva: <Veeva />,
        comingSoon: <More />,
    }
    return (
        <div className={classes.container} onClick={() => props.navClick()}>
            <div className={classes.img}>
                {tasks[props.task]}
            </div>
        </div>
    )
}