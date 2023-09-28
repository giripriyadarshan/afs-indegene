import classes from './LeftNavTask.module.css';
import Veeva from '../../assets/leftNavIcons/veeva-icon-black.svg';
import More from '../../assets/leftNavIcons/more-horizontal.svg';
import { Tooltip } from 'react-tooltip';
import { useState } from 'react';


/* 
    Deploys task
    tasks are defined in a key value pair below
        the key is the name of the task
        the value is the icon which will be displayed
    the task is deployed on click and the event is bubbled up to the parent component
*/
export default function LeftNavTask(props) {
    const tasks = {
        veeva: Veeva,
        comingSoon: More,
    }

    const tooltipText = {
        veeva: 'Veeva Vault (All Instances)',
        comingSoon: 'Coming Soon',
    }

    const [tooltipVisibility, setTooltipVisibility] = useState(false);

    return (
        <div
            className={`
            ${classes.container} 
            ${classes[props.activeTask == props.task ? 'true' : 'false']}
        `}
            onClick={() => {
                props.navClick()
                setTooltipVisibility(false)
            }}
            onMouseEnter={() => setTooltipVisibility(true)}
            onMouseLeave={() => setTooltipVisibility(false)}

            data-tooltip-id={props.task}
            data-tooltip-content={tooltipText[props.task]}
            data-tooltip-place="right"
        >
            <div className={classes.img}>
                <img src={tasks[props.task]} alt="" className={classes[props.task]} />
            </div>
            <Tooltip id={props.task} className={classes.tooltip} isOpen={tooltipVisibility} />
        </div>
    )
}