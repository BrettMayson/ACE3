class SPE_Truck_base {
    EGVAR(cargo,hasCargo) = 1;
    EGVAR(cargo,space) = 4;
    EGVAR(refuel,canReceive) = 1;
    EGVAR(refuel,fuelCapacity) = 88;
    EGVAR(vehicle_damage,hullDetonationProb) = 0.2;
    EGVAR(vehicle_damage,turretDetonationProb) = 0.03;
    EGVAR(vehicle_damage,engineDetonationProb) = 0.03;
    EGVAR(vehicle_damage,hullFireProb) = 0.6;
    EGVAR(vehicle_damage,turretFireProb) = 0.1;
    EGVAR(vehicle_damage,engineFireProb) = 0.2;
    EGVAR(vehicle_damage,detonationDuringFireProb) = 0.2;
    EGVAR(vehicle_damage,canHaveFireRing) = 0.1;
};
class SPE_WheeledTracked_APC_base: SPE_Truck_base {
    EGVAR(refuel,fuelCapacity) = 140;
    EGVAR(vehicle_damage,hullDetonationProb) = 0.03;
    EGVAR(vehicle_damage,turretDetonationProb) = 0.03;
    EGVAR(vehicle_damage,engineDetonationProb) = 0.03;
    EGVAR(vehicle_damage,hullFireProb) = 0.3;
    EGVAR(vehicle_damage,turretFireProb) = 0.1;
    EGVAR(vehicle_damage,engineFireProb) = 0.2;
    EGVAR(vehicle_damage,detonationDuringFireProb) = 0.2;
    EGVAR(vehicle_damage,canHaveFireRing) = 0.1;
};

class SPE_Halftrack_base: SPE_WheeledTracked_APC_base {
    EGVAR(refuel,fuelCapacity) = 230;
    EGVAR(vehicle_damage,hullDetonationProb) = 0.03;
    EGVAR(vehicle_damage,turretDetonationProb) = 0.03;
    EGVAR(vehicle_damage,engineDetonationProb) = 0.03;
    EGVAR(vehicle_damage,hullFireProb) = 0.3;
    EGVAR(vehicle_damage,turretFireProb) = 0.1;
    EGVAR(vehicle_damage,engineFireProb) = 0.2;
    EGVAR(vehicle_damage,detonationDuringFireProb) = 0.2;
    EGVAR(vehicle_damage,canHaveFireRing) = 0.1;
};


// W H E E L E D - AXIS
class SPE_OpelBlitz_base: SPE_Truck_base {
    EGVAR(refuel,fuelCapacity) = 88;
};

class SPE_OpelBlitz_Fuel: SPE_OpelBlitz_base {
    transportFuel = 0;
    EGVAR(refuel,hooks)[] = {{-0.23,-2.58,-0.59}};
    EGVAR(refuel,fuelCargo) = 2000;
};

class SPE_OpelBlitz_Ambulance: SPE_OpelBlitz_base {
    EGVAR(medical,medicClass) = 1;
};

class SPE_OpelBlitz_Repair: SPE_OpelBlitz_base {
    transportRepair = 0;
    EGVAR(repair,canRepair) = 1;
    transportAmmo = 0;
    EGVAR(rearm,defaultSupply) = 1200;
};

class SPE_OpelBlitz_Ammo: SPE_OpelBlitz_base {
    transportAmmo = 0;
    EGVAR(rearm,defaultSupply) = 1200;
};

// W H E E L E D ALLIED FORCES

class SPE_US_M3_Halftrack_Fuel: SPE_Halftrack_base {
    transportFuel = 0;
    EGVAR(refuel,hooks)[] = {{-0.23,-2.58,-0.59}};
    EGVAR(refuel,fuelCargo) = 2000;
};

class SPE_US_M3_Halftrack_Ambulance: SPE_Halftrack_base {
    EGVAR(medical,medicClass) = 1;
};

class SPE_US_M3_Halftrack_Repair: SPE_Halftrack_base {
    transportRepair = 0;
    EGVAR(repair,canRepair) = 1;
};

class SPE_US_M3_Halftrack_Ammo: SPE_Halftrack_base {
    transportAmmo = 0;
    EGVAR(rearm,defaultSupply) = 1200;
};