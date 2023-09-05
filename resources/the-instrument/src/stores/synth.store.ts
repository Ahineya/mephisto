import {StoreSubject} from "@dgaa/store-subject";



class SynthStore {
    public chart = new StoreSubject("");

    constructor() {

    }

    setChart(chart: string) {
        this.chart.next(chart);
    }
}

export const synthStore = new SynthStore();
