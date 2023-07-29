

var categories = ['alkali-metals', 'alkaline-earth-metals', 'lanthanoids', 'actinoids', 'transition-metals', 'post-transition-metals', 'metalloids', 'other-nonmetals', 'noble-gasses', 'unknown'];



  function element(number, symbol, name, mass, weight, material, column, row, targetPage) {
    var elementHtml = `
      <div class="element ${material} c${column} r${row}">
        <a href="${targetPage}">
          <div class="square">
            <div class="model">
              ${weight.reverse().map((item) => `
                <div class="orbital">
                  ${Array(item).fill().map(() => '<div class="electron"></div>').join('')}
                </div>
              `).join('')}
            </div>
            <div class="atomic-number">${number}</div>
            <div class="label">
              <div class="symbol">${symbol}</div>
              <div class="name">${name}</div>
            </div>
            <div class="atomic-mass">${mass}</div>
            <ul class="atomic-weight">
              ${weight.reverse().map((item) => `<li>${item}</li>`).join('')}
            </ul>
          </div>
        </a>
      </div>
    `;
    return elementHtml;
  }




  function placeholder(number, material, column, row) {
    var placeholderHtml = `
      <div class="placeholder ${material} c${column} r${row}">
        <div class="square">${number}</div>
      </div>
    `;
    return placeholderHtml;
  }



