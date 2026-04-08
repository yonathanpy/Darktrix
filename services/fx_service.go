package services

import "quorix-bridge/internal/models"

var SupportedCurrencies = []models.Currency{
	{Code: "USD", Rate: 1.0, Name: "US Dollar", Symbol: "$"},
	{Code: "ETB", Rate: 57.2, Name: "Ethiopian Birr", Symbol: "Br"},
	{Code: "BTC", Rate: 65000.0, Name: "Bitcoin", Symbol: "₿"},
}

func GetRate(from, to string) (float64, error) {
	var fromRate, toRate float64
	foundFrom, foundTo := false, false

	for _, c := range SupportedCurrencies {
		if c.Code == from {
			fromRate = c.Rate
			foundFrom = true
		}
		if c.Code == to {
			toRate = c.Rate
			foundTo = true
		}
	}

	if !foundFrom || !foundTo {
		return 0, fmt.Errorf("currency not supported")
	}
	return toRate / fromRate, nil
}
