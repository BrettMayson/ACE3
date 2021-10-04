[
    QGVAR(convertExplosives), "CHECKBOX",
    [LSTRING(convertExplosives_DisplayName), LSTRING(convertExplosives_Description)],
    LSTRING(Settings_DisplayName),
    true,
    true,
    {},
    true
] call CBA_fnc_addSetting;

[
    QGVAR(flashbangSurrender),
    "CHECKBOX",
    [LSTRING(flashbangSurrender_DisplayName), LSTRING(flashbangSurrender_Description)],
    LSTRING(Settings_DisplayName),
    false,
    true
] call CBA_fnc_addSetting;

[
    QGVAR(flashbangSurrenderChance),
    "SLIDER",
    [LSTRING(flashbangSurrenderChance_DisplayName), LSTRING(flashbangSurrenderChance_Description)],
    LSTRING(Settings_DisplayName),
    [0, 1, 0.5, 2, true],
    true
] call CBA_fnc_addSetting;

[
    QGVAR(flashbangSurrenderTimeout),
    "SLIDER",
    [LSTRING(flashbangSurrenderTimeout_DisplayName), LSTRING(flashbangSurrenderTimeout_Description)],
    LSTRING(Settings_DisplayName),
    [30, 600, 300, 0],
    true
] call CBA_fnc_addSetting;
