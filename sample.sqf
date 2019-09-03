#define PRINT(text) systemChat text

private _a = 4;
private _b = _a + 2;
PRINT(str _b);

if (_b == 1) then {
	systemChat "Things have gone ""wrong""";
} else {
	systemChat "Things seem to be ok";
	if (_a == 4) then {
		PRINT("neat");
	};
};

if (_b == 6) then {
	systemChat "The VM works";
};

while { _b < 10 } do {
	_b = _b + 1;
};
PRINT("All good!");
systemChat "PRINT(""Real Good"")";
