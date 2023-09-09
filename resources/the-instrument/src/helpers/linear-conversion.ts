import {scalePow} from "d3-scale";

export const linearConversion = (value: number, fromStart: number, fromEnd: number, toStart: number, toEnd: number): number => {
  return ((value - fromStart) / (fromEnd - fromStart)) * (toEnd - toStart) + toStart;
}

export const createConversionFunctions = (conversionType: 'linear' | 'exponential', fromStart: number, fromEnd: number, toStart: number, toEnd: number) => {
  switch (conversionType) {
    case "exponential":
      const expScale = scalePow()
        .exponent(3)
        .domain([0, 270])
        .range([fromStart, fromEnd]);

      return {
        convertTo: expScale,
        convertFrom: expScale.invert.bind(expScale)
      }
    case "linear":
      return {
        convertTo: (value: number) => linearConversion(value, toStart, toEnd, fromStart, fromEnd),
        convertFrom: (value: number) => linearConversion(value, fromStart, fromEnd, toStart, toEnd),
      }
  }
}

