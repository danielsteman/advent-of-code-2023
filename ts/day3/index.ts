var fs = require("fs");

class Engine {
  private grid: string;
  private reprGrid: string;

  constructor(inputFile: string) {
    this.reprGrid = fs.readFileSync(inputFile).toString("utf-8");
    this.grid = this.reprGrid.replace(/\s/g, "");
  }

  toString(): string {
    return this.reprGrid;
  }

  static isAlphaNumericOrDot(str: string) {
    var code: number, i: number, len: number;

    for (i = 0, len = str.length; i < len; i++) {
      code = str.charCodeAt(i);
      if (
        !(code > 47 && code < 58) && // numeric (0-9)
        !(code > 64 && code < 91) && // upper alpha (A-Z)
        !(code > 96 && code < 123) && // lower alpha (a-z)
        !(code == 46) // dot
      ) {
        return false;
      }
    }
    return true;
  }
}

const engine = new Engine("./input.txt");
console.log(engine.toString());
