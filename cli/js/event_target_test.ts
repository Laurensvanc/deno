// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
import { test, assert } from "./test_util.ts";

test(function addEventListenerTest(): void {
  const document = new EventTarget();

  // @ts-ignore tests ignoring the type system for resilience
  assert.equals(document.addEventListener("x", null, false), undefined);
  // @ts-ignore
  assert.equals(document.addEventListener("x", null, true), undefined);
  // @ts-ignore
  assert.equals(document.addEventListener("x", null), undefined);
});

test(function constructedEventTargetCanBeUsedAsExpected(): void {
  const target = new EventTarget();
  const event = new Event("foo", { bubbles: true, cancelable: false });
  let callCount = 0;

  const listener = (e: Event): void => {
    assert.equals(e, event);
    ++callCount;
  };

  target.addEventListener("foo", listener);

  target.dispatchEvent(event);
  assert.equals(callCount, 1);

  target.dispatchEvent(event);
  assert.equals(callCount, 2);

  target.removeEventListener("foo", listener);
  target.dispatchEvent(event);
  assert.equals(callCount, 2);
});

test(function anEventTargetCanBeSubclassed(): void {
  class NicerEventTarget extends EventTarget {
    on(
      type: string,
      callback: (e: Event) => void | null,
      options?: __domTypes.AddEventListenerOptions
    ): void {
      this.addEventListener(type, callback, options);
    }

    off(
      type: string,
      callback: (e: Event) => void | null,
      options?: __domTypes.EventListenerOptions
    ): void {
      this.removeEventListener(type, callback, options);
    }
  }

  const target = new NicerEventTarget();
  new Event("foo", { bubbles: true, cancelable: false });
  let callCount = 0;

  const listener = (): void => {
    ++callCount;
  };

  target.on("foo", listener);
  assert.equals(callCount, 0);

  target.off("foo", listener);
  assert.equals(callCount, 0);
});

test(function removingNullEventListenerShouldSucceed(): void {
  const document = new EventTarget();
  // @ts-ignore
  assert.equals(document.removeEventListener("x", null, false), undefined);
  // @ts-ignore
  assert.equals(document.removeEventListener("x", null, true), undefined);
  // @ts-ignore
  assert.equals(document.removeEventListener("x", null), undefined);
});

test(function constructedEventTargetUseObjectPrototype(): void {
  const target = new EventTarget();
  const event = new Event("toString", { bubbles: true, cancelable: false });
  let callCount = 0;

  const listener = (e: Event): void => {
    assert.equals(e, event);
    ++callCount;
  };

  target.addEventListener("toString", listener);

  target.dispatchEvent(event);
  assert.equals(callCount, 1);

  target.dispatchEvent(event);
  assert.equals(callCount, 2);

  target.removeEventListener("toString", listener);
  target.dispatchEvent(event);
  assert.equals(callCount, 2);
});

test(function toStringShouldBeWebCompatible(): void {
  const target = new EventTarget();
  assert.equals(target.toString(), "[object EventTarget]");
});

test(function dispatchEventShouldNotThrowError(): void {
  let hasThrown = false;

  try {
    const target = new EventTarget();
    const event = new Event("hasOwnProperty", {
      bubbles: true,
      cancelable: false
    });
    const listener = (): void => {};
    target.addEventListener("hasOwnProperty", listener);
    target.dispatchEvent(event);
  } catch {
    hasThrown = true;
  }

  assert.equals(hasThrown, false);
});

test(function eventTargetThisShouldDefaultToWindow(): void {
  const {
    addEventListener,
    dispatchEvent,
    removeEventListener
  } = EventTarget.prototype;
  let n = 1;
  const event = new Event("hello");
  const listener = (): void => {
    n = 2;
  };

  addEventListener("hello", listener);
  window.dispatchEvent(event);
  assert.equals(n, 2);
  n = 1;
  removeEventListener("hello", listener);
  window.dispatchEvent(event);
  assert.equals(n, 1);

  window.addEventListener("hello", listener);
  dispatchEvent(event);
  assert.equals(n, 2);
  n = 1;
  window.removeEventListener("hello", listener);
  dispatchEvent(event);
  assert.equals(n, 1);
});
