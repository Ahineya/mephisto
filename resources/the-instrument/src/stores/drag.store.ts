import {BehaviorSubject, Subject} from "rxjs";

class DragStore {
  public onDragStart = new BehaviorSubject<string | null>(null);
  public onDragEnd = new Subject();

  public onDragMoveDiffChanged = new Subject<number>();

  public startDrag(id: string) {
    this.onDragStart.next(id);
  }

  public endDrag() {
    this.onDragEnd.next(null);
    this.onDragStart.next(null);
    this.onDragMoveDiffChanged.next(0);
  }

  public changeDragMoveDiff(diff: number) {
    if (diff === 0) {
      return;
    }

    this.onDragMoveDiffChanged.next(diff);
  }
}

export const dragStore = new DragStore();
