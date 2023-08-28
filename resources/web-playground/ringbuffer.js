/*
        stdlib.insert("buf_new".to_string(), "new Ringbuffer".to_string());
        stdlib.insert("buf_read".to_string(), "Ringbuffer.read".to_string());
        stdlib.insert("buf_push".to_string(), "Ringbuffer.push".to_string());
        stdlib.insert("buf_pop".to_string(), "Ringbuffer.pop".to_string());
        stdlib.insert("buf_length".to_string(), "Ringbuffer.length".to_string());
        stdlib.insert("buf_clear".to_string(), "Ringbuffer.clear".to_string());
        stdlib.insert("buf_put".to_string(), "Ringbuffer.put".to_string());
 */

const Rb = {
    read: function (rb, index) {
        return rb.get(index);
    },

    push: function (rb, element) {
        rb.push(element);
    },

    pop: function (rb) {
        return rb.pop();
    },

    length: function (rb) {
        return rb.length;
    },

    clear: function (rb) {
        rb.clear();
    },

    put: function (rb, index, value) {
        rb.set(index, value);
    },

    setAll: function (rb, fn) {
        rb.setAll(fn);
    },

    resize: function (rb, size) {
        rb.resize(size);
    }
}


class Ringbuffer {
    constructor(size) {
        this.elements = new Float64Array(size);
        this.readIndex = 0;
        this.writeIndex = 0;
        console.log(this.elements);
    }

    push(element) {

        // This is super slow. It should be ran in the realtime audio thread.
        // for (let i = 0; i < this.elements.length - 1; i++) {
        //     this.elements[i] = this.elements[i + 1];
        // }
        //
        // this.elements[this.elements.length - 1] = element;

        // This is faster, but it's not a ringbuffer.
        // this.elements.push(element);

        // This is the fastest
        this.elements[this.writeIndex] = element;
        this.writeIndex++;

        if (this.writeIndex >= this.elements.length) {
            this.writeIndex = 0;
        }

        if (this.writeIndex === this.readIndex) {
            this.readIndex++;

            if (this.readIndex >= this.elements.length) {
                this.readIndex = 0;
            }
        }
    }

    pop() {
        // This is super slow. It should be ran in the realtime audio thread.
        // const element = this.elements[0];
        //
        // for (let i = 0; i < this.elements.length - 1; i++) {
        //     this.elements[i] = this.elements[i + 1];
        // }
        //
        // this.elements[this.elements.length - 1] = 0;
        //
        // return element;

        // And here goes the proper implementation
        const element = this.elements[this.readIndex];
        this.readIndex++;

        if (this.readIndex >= this.elements.length) {
            this.readIndex = 0;
        }

        return element;
    }

    peek() {
        // Peek into the current element
        return this.elements[this.readIndex];
    }

    get(index) {
        // Get the element at the given index, starting from the read index
        return this.elements[(this.readIndex + index) % this.elements.length];
    }

    set(index, value) {
        // Set the element at the given index, starting from the read index
        this.elements[(this.readIndex + index) % this.elements.length] = value;
    }

    setAll(fn) {
        for (let i = 0; i < this.elements.length; i++) {
            Rb.push(this, fn(i));
        }
    }

    resize(size) {
        this.elements = new Float64Array(size);
        this.readIndex = 0;
        this.writeIndex = 0;
    }

    clear() {
        for (let i = 0; i < this.elements.length; i++) {
            this.elements[i] = 0;
        }
    }

    get length() {
        return this.elements.length;
    }
}

