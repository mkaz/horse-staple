
pub fn words() -> Vec<&'static str> {

	let w = "Alabama
	Alameda
	Alaska
	Albany
	America
	Anchorage
	Annapolis
	Antioch
	Antonio
	Arcadia
	Arizona
	Arkansas
	Arlington
	Atherton
	Atlanta
	Auburn
	Augusta
	Austin
	Avalon
	Avenal
	Baker
	Bakersfield
	Baldwin
	Baltimore
	Barstow
	BatonRouge
	Beaumont
	Bell
	Bellflower
	Belmont
	Beverly
	BigBearLake
	Billings
	Birmingham
	Bishop
	Blind
	BlueLake
	Boise
	Bombers
	Boston
	Brandon
	Brenda
	Brentwood
	Bridgeport
	Brisbane
	Brooklyn
	BuenaPark
	Burbank
	Burlingame
	Burlington
	California
	Calistoga
	Calvin
	Campbell
	Canyon
	CanyonLake
	Carmel
	Carson
	CarsonCity
	Charleston
	Charlotte
	Chester
	Chicago
	Chico
	Chino
	ChinoHills
	CitrusHeights
	Claremont
	Clayton
	Clearlake
	Clinton
	Cloverdale
	Clovis
	Colfax
	Colma
	Colorado
	Colton
	Columbia
	Columbus
	Compton
	Concord
	Connect
	Coolidge
	Corning
	Corolla
	Corona
	Coronado
	CorteMadera
	CostaMesa
	Covina
	CulverCity
	Cupertino
	DalyCity
	DanaPoint
	Danville
	Davis
	Delaware
	Denver
	DesMoines
	Desert
	Detroit
	Dixon
	Dover
	Downey
	ElMonte
	ElPaso
	ElSegundo
	ElkGrove
	Emeryville
	Eugene
	Eureka
	Fairfax
	Fairfield
	Falls
	Fargo
	Farmersville
	Ferndale
	Florida
	Folsom
	Fontana
	FortJones
	FortWorth
	Fortuna
	FosterCity
	Frankfort
	Franklin
	Fremont
	Fresno
	Fullerton
	GardenGrove
	Gardena
	Gardens
	Garfield
	Georgia
	Gilroy
	Glendale
	Glendora
	GrassValley
	Greenfield
	Grover
	HalfMoonBay
	Hamilton
	Harrisburg
	Harrison
	Hartford
	Hawaii
	Hawthorne
	Hayward
	Helena
	Hercules
	Highland
	Hills
	Honolulu
	Houston
	Idaho
	India
	Indiana
	Industry
	Iowa
	Irvine
	Jackson
	Jacksonville
	Jefferson
	Jersey
	Kansas
	KansasCity
	Kentucky
	KingCity
	Kingsburg
	LaMesa
	LaPalma
	LagunaBeach
	LagunaHills
	LagunaWoods
	LakeForest
	Lakeport
	Lakewood
	Lancaster
	Lansing
	Larkspur
	LasVegas
	Lawndale
	LemonGrove
	Lincoln
	LittleRock
	LiveOak
	Livermore
	Lodi
	LongBeach
	Loomis
	LosAlamitos
	LosAltos
	LosAngeles
	LosBanos
	LosGatos
	Louisiana
	Louisville
	Lynwood
	Madison
	Maine
	Malibu
	Manchester
	Manteca
	Martinez
	Maryland
	Marysville
	Maywood
	McKinley
	Memphis
	MenloPark
	Merced
	Mesa
	Mets
	Mexico
	Miami
	Michigan
	MillValley
	Millbrae
	Milpitas
	Milwaukee
	Minnesota
	Mississippi
	Modesto
	Monroe
	Montana
	Montgomery
	Moorpark
	MorganHill
	MountShasta
	MountainView
	Napa
	Nashville
	Nebraska
	Needles
	Nevada
	NevadaCity
	NewHampshire
	NewJersey
	NewMexico
	NewOrleans
	NewYork
	Newark
	Newman
	NewportBeach
	NorthDakota
	Norwalk
	Novato
	Oakdale
	Oakland
	Ohio
	Oklahoma
	Olympia
	Omaha
	Oregon
	Orlando
	Oroville
	Oxnard
	PacificGrove
	Pacifica
	PalmDesert
	PalmSprings
	Palmdale
	PaloAlto
	PalosVerdes
	Paradise
	Paramount
	Pasadena
	PasoRobles
	Petaluma
	Philadelphia
	Phoenix
	Piedmont
	PismoBeach
	Pittsburg
	Placerville
	PleasantHill
	Pleasanton
	Polk
	Portland
	Predators
	Providence
	RanchoCordova
	Ranger
	Rapids
	RedBluff
	Redding
	Redlands
	RedwoodCity
	Reno
	Revolution
	RhodeIsland
	Richmond
	Ridgecrest
	Ridgeline
	RioDell
	RioVista
	Riverbank
	Riverside
	Rockies
	Rocklin
	RollingHills
	Roosevelt
	Rosemead
	Roseville
	Rutherford
	Sacramento
	Salem
	Salinas
	SaltLakeCity
	SanAnselmo
	SanAntonio
	SanBruno
	SanCarlos
	SanClemente
	SanDiego
	SanDimas
	SanFernando
	SanFrancisco
	SanJose
	SanLeandro
	SanLuisObispo
	SanMarcos
	SanMarino
	SanMateo
	SanPablo
	SanRamon
	SandCity
	SantaAna
	SantaBarbara
	SantaClara
	SantaCruz
	SantaFe
	SantaMaria
	SantaMonica
	SantaPaula
	SantaRosa
	Saratoga
	ScottsValley
	SealBeach
	Seaside
	Seattle
	Selma
	Shasta
	SimiValley
	Solvang
	Sonoma
	Sonora
	SouthCarolina
	SouthDakota
	Springfield
	StPaul
	StHelena
	Stanton
	Stockton
	Sunnyvale
	Susanville
	SutterCreek
	Taft
	TempleCity
	Texans
	Texas
	ThousandOaks
	Thunder
	Timbers
	Timberwolves
	Titans
	Toledo
	Topeka
	Torrance
	Tracy
	Trail
	Transit
	Trenton
	Trinidad
	Truckee
	Truman
	Tucker
	Tulsa
	Turlock
	Twins
	Tyler
	USA
	Union
	UnionCity
	United
	Upland
	Utah
	Vacaville
	Vallejo
	Valley
	VanBuren
	Ventura
	Vermont
	Vernon
	Victoria
	Victorville
	VillaPark
	Virginia
	VirginiaBeach
	Walnut
	WalnutCreek
	Washington
	Waterford
	Watsonville
	Wayne
	Weed
	WestCovina
	WestHollywood
	WestSacramento
	WestVirginia
	Westlake
	Westminster
	Westmorland
	Wheatland
	Wheelchair
	Whitecaps
	Whittier
	Wichita
	Willows
	Wilmington
	Wilson
	Windsor
	Wings
	Winters
	Wisconsin
	Wizards
	Woodlake
	Woodland
	Woodrow
	Woodside
	Wyoming
	YorbaLinda
	Yountville
	YubaCity
	Zachary";

	return w.lines().collect();
}