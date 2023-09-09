import {useEffect, useState} from "react";
import classNames from "classnames";
import "./drag-overlay.scss";
import {dragStore} from "../../stores/drag.store.ts";

export const DragOverlay = () => {

  const [isDragging, setIsDragging] = useState(false);

  useEffect(() => {
    const subscriptions = [
      dragStore.onDragStart.subscribe((id) => {
        if (!id) {
          return;
        }

        setIsDragging(true);
      }),
      dragStore.onDragEnd.subscribe(() => {
        setIsDragging(false);
      })
    ];

    return () => subscriptions.forEach(s => s.unsubscribe());
  }, []);

  useEffect(() => {
    if (isDragging) {
      window.addEventListener('mousemove', updateDrag);
      window.addEventListener('mouseup', stopDrag);
    }

    return () => {
        window.removeEventListener('mousemove', updateDrag);
        window.addEventListener('mouseup', stopDrag);
    }
  }, [isDragging]);

  const stopDrag = () => {
    dragStore.endDrag();
  }

  const updateDrag = (e: MouseEvent) => {
    dragStore.changeDragMoveDiff(-e.movementX);
  }

  return <div className={classNames("drag-overlay", {"dragging": isDragging})}/>
}