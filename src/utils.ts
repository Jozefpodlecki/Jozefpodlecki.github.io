export const randomBorder = () => {
    const random = () => Math.floor(Math.random() * 200 + 100);
    const topLeftX = random()
    const topLeffY = random()
    const topRightX = random()
    const topRightY = random()
    const bottomRightX = random()
    const bottomRightY = random()
    const bottomLeftX = random()
    const bottomLeftY = random()
    return `border-radius: ${topLeftX}px ${topRightX}px ${bottomRightX}px ${bottomLeftX}px / ${topLeffY}px ${topRightY}px ${bottomRightY}px ${bottomLeftY}px`
  }

 export const addAnimation = () => {
    const style = document.createElement('style');
    style.type = 'text/css';
    let animation = '';

    for(let it of Array(100).fill(0).map((pr, index) => index)) {
      const property = randomBorder();
      animation += `${it}% {${property}}`;
    }

    let computedStyle = `
    .randomBorder {
      animation: randomBorder 100s infinite forwards;
    }
    @keyframes randomBorder {
      ${animation}
    }
    @-webkit-keyframes randomBorder {
      ${animation}
    }
    @-moz-keyframes randomBorder {
      ${animation}
    }`;
    style.innerHTML = computedStyle;
    document.getElementsByTagName('head')[0].appendChild(style);
  }