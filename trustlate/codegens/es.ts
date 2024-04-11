export const trustlate = {
  secondaryPage: {
    greeting: (name: string, surname: string) => `Hola ${name} ${surname}`,
  },
  mainPage: { subTitle: "Mundo", title: "Hola" },
  thirdPage: {
    one: "Uno",
    numbers: (num1: string, num2: string, num3: string) =>
      `${num1}, ${num2}, ${num3}`,
  },
} as const;
