export {};

declare global {
    interface EventWithTarget<T> extends Event {
        target : T & EventTarget
    }
}