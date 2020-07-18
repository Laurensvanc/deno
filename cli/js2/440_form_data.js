// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.

((window) => {
  const blob = window.__blob;
  const domFile = window.__domFile;
  const { DomIterableMixin } = window.__domIterable;
  const { requiredArguments } = window.__webUtil;

  const dataSymbol = Symbol("data");

  class FormDataBase {
    [dataSymbol] = [];

    append(
      name,
      value,
      filename,
    ) {
      requiredArguments("FormData.append", arguments.length, 2);
      name = String(name);
      if (value instanceof domFile.DomFile) {
        this[dataSymbol].push([name, value]);
      } else if (value instanceof blob.Blob) {
        const dfile = new domFile.DomFile([value], filename || "blob", {
          type: value.type,
        });
        this[dataSymbol].push([name, dfile]);
      } else {
        this[dataSymbol].push([name, String(value)]);
      }
    }

    delete(name) {
      requiredArguments("FormData.delete", arguments.length, 1);
      name = String(name);
      let i = 0;
      while (i < this[dataSymbol].length) {
        if (this[dataSymbol][i][0] === name) {
          this[dataSymbol].splice(i, 1);
        } else {
          i++;
        }
      }
    }

    getAll(name) {
      requiredArguments("FormData.getAll", arguments.length, 1);
      name = String(name);
      const values = [];
      for (const entry of this[dataSymbol]) {
        if (entry[0] === name) {
          values.push(entry[1]);
        }
      }

      return values;
    }

    get(name) {
      requiredArguments("FormData.get", arguments.length, 1);
      name = String(name);
      for (const entry of this[dataSymbol]) {
        if (entry[0] === name) {
          return entry[1];
        }
      }

      return null;
    }

    has(name) {
      requiredArguments("FormData.has", arguments.length, 1);
      name = String(name);
      return this[dataSymbol].some((entry) => entry[0] === name);
    }

    set(
      name,
      value,
      filename,
    ) {
      requiredArguments("FormData.set", arguments.length, 2);
      name = String(name);

      // If there are any entries in the context object’s entry list whose name
      // is name, replace the first such entry with entry and remove the others
      let found = false;
      let i = 0;
      while (i < this[dataSymbol].length) {
        if (this[dataSymbol][i][0] === name) {
          if (!found) {
            if (value instanceof domFile.DomFile) {
              this[dataSymbol][i][1] = value;
            } else if (value instanceof blob.Blob) {
              this[dataSymbol][i][1] = new domFile.DomFile(
                [value],
                filename || "blob",
                {
                  type: value.type,
                },
              );
            } else {
              this[dataSymbol][i][1] = String(value);
            }
            found = true;
          } else {
            this[dataSymbol].splice(i, 1);
            continue;
          }
        }
        i++;
      }

      // Otherwise, append entry to the context object’s entry list.
      if (!found) {
        if (value instanceof domFile.DomFile) {
          this[dataSymbol].push([name, value]);
        } else if (value instanceof blob.Blob) {
          const dfile = new domFile.DomFile([value], filename || "blob", {
            type: value.type,
          });
          this[dataSymbol].push([name, dfile]);
        } else {
          this[dataSymbol].push([name, String(value)]);
        }
      }
    }

    get [Symbol.toStringTag]() {
      return "FormData";
    }
  }

  class FormData extends DomIterableMixin(FormDataBase, dataSymbol) {}

  window.__formData = {
    FormData,
  };
})(this);
