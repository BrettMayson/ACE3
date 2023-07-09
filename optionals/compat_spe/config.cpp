#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        name = COMPONENT_NAME;
        units[] = {
            "ACE_SPE_MLE_27_31_Stand_Deployed",
            "ACE_SPE_GrW278_1_Stand_Deployed",
            "ACE_SPE_M1_81_Stand_Deployed",
            "ACE_SPE_MG42_Tripod_Disasm",
            "ACE_SPE_M2_Tripod_Bag"
        };
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {
            "cba_main",
            "ace_csw",
            "ace_explosives",
            "ace_rearm",
            "ace_refuel",
            "ace_repair",
            "ww2_spe_assets_c_weapons_infantryweapons_c",
            "ww2_spe_assets_c_vehicles_staticweapons_c",
            "ww2_spe_assets_c_vehicles_weapons_c",
            "ww2_spe_core_f_system_staticweapons_f"
        };
        author = ECSTRING(common, ACETeam);
        authors[] = {
            "sancron",
            "nomisum",
            "coldfront15/Henderson",
            "BrettMayson"
        };
        url = ECSTRING(main, URL);
        VERSION_CONFIG;
    };
};

#include "ACE_CSW_Groups.hpp"
#include "ACE_Medical_Injuries.hpp"
#include "CfgAmmo.hpp"
#include "CfgEventHandlers.hpp"
#include "CfgGlasses.hpp"
#include "CfgMagazines.hpp"
#include "CfgVehicles.hpp"
#include "CfgWeapons.hpp"