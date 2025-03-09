document.addEventListener("DOMContentLoaded", () => {
    const cryptoList = document.getElementById("crypto-list");
    const themeToggle = document.getElementById("theme-toggle");

    if (!cryptoList || !themeToggle) {
        console.error("Ошибка: элементы не найдены. Проверь, загружен ли HTML.");
        return;
    }

    if (localStorage.getItem("theme") === "dark") {
        document.body.classList.add("dark-theme");
    }

    async function fetchCryptoData() {
        try {
            const response = await fetch("/api/crypto");
            if (!response.ok) throw new Error(`Ошибка HTTP: ${response.status}`);
            const data = await response.json();
            console.log("Полученные данные: ", data);

            cryptoList.innerHTML = "";
            for (const [name, info] of Object.entries(data)) {
                const price = info.usd.toFixed(2);
                const change = info.usd_24h_change ? info.usd_24h_change.toFixed(2) : "0.00";
                const changeClass = change >= 0 ? "up" : "down";

                const card = document.createElement("div");
                card.className = "crypto-card";
                card.innerHTML = `
                    <h3>${name.toUpperCase()}</h3>
                    <p>Цена: $${price}</p>
                    <p class="price-change ${changeClass}">${change}%</p>
                `;
                cryptoList.appendChild(card);
            }
        } catch (error) {
            console.error("Ошибка загрузки данных:", error);
        }
    }

    fetchCryptoData();
    setInterval(fetchCryptoData, 60000);

    themeToggle.addEventListener("click", () => {
        document.body.classList.toggle("dark-theme");
        if (document.body.classList.contains("dark-theme")) {
            localStorage.setItem("theme", "dark");
        } else {
            localStorage.setItem("theme", "light");
        }
    });
});
