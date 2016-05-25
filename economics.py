#! /usr/bin/env python3

cost = {}
seconds_per_year = 365.25 * 24 * 3600
### BEGIN RAW FOOD COSTS #############################################
def placentalia_power(kg):
	"""Use the formula 70*kg**(0.75) to compute energy requirement per day"""
	return 4184*70*kg**(0.75) / 3600 / 24

raccoon_min_mass = 3.5
raccoon_max_mass = 9  # source: -
raccoon_count = 30
raccoon_mass = (raccoon_min_mass + raccoon_max_mass) / 2
food_unit_price = 30  # source: -
food_unit_energy = 4184*3585  # source: http://www.pfma.org.uk/_assets/docs/fact-sheet/PFMA-fact-sheet-calories.pdf

raccoon_power = placentalia_power(raccoon_mass)
raccoon_energy_per_year = raccoon_power * seconds_per_year
food_per_year_per_raccoon = raccoon_energy_per_year / food_unit_energy
money_per_year_per_raccoon = food_per_year_per_raccoon * food_unit_price
money_for_food_total = raccoon_count * money_per_year_per_raccoon

cost["food"] = raccoon_count * money_per_year_per_raccoon
### END RAW FOOD COSTS ###############################################
### BEGIN HEALTH COSTS ###############################################
flea_cure_cost = 36  # Use three pounds per cure, converts to 36 NOK. Source: http://shop.rspca.org.uk/pet-food-accessories-wildlife/flea-tick-treatments.html
cost["health"] = flea_cure_cost * raccoon_count

euthanization_cost = 100  # By dry ice or nitrogen gas
kill_ratio = 1/3  # Kill so many of the live raccoons each year
cost["euthanization"] = kill_ratio * raccoon_count * euthanization_cost
### END HEALTH COSTS #################################################
### BEGIN UTILITY COSTS ##############################################
power_consumption = 300  # Local machine for monitoring
price_per_kwh = 0.2816  # post code 7012. Source: http://www.fjordkraft.no/privat/strompriser/
price_per_joule = price_per_kwh / 1000 / 3600
cost["power"] = power_consumption * seconds_per_year * price_per_joule
### END UTILITY COSTS ################################################
### BEGIN WEB COSTS ##################################################
website_per_year = 100
domain_per_year = 150
cost["website"] = website_per_year + domain_per_year
### END WEB COSTS ####################################################
### BEGIN REPAIR COST ################################################
cost["protection"] = 300*3  # Assuming three pairs per year. Source: http://www.sikkerhetsakademiet.no/produkt/stikksikre-hansker/
### END REPAIR COST ##################################################
### BEGIN FIXED MATERIAL COST ########################################
wood_per_cage = 3*4 + 2*4 + 1.5*4  # 3x2x1.5 (x,y,z)
cost_per_meter = 13.95  # Source: https://www.byggmax.no/treverk/impregnert-trevirke/impregnerte-rekker/impr.-konstruksjonsvirke-48x98-gr%C3%B8nn-p08748098
cages = raccoon_count

base_area_cage = 3.1*2.1  # Added padding because cages need to be spaced
total_area = base_area_cage * cages

cage_area = 3*2*2 + 3*1.5*2 + 2*1.5*2
chicken_wire_price_per_area = 539.95 / (1.5*30)  # Source: http://hegnslageret.dk/hoensetraad-50x50mm-masker-126/

bin_cost = 80  # So the raccoons can sleep in an enclosed space. Source: http://www.clasohlson.com/no/Papirkurv/31-4340-1

cost["fixed-construction"] = wood_per_cage * cages * cost_per_meter
cost["fixed-fence"] = cage_area * cages * chicken_wire_price_per_area
cost["fixed-bins"] = cages * bin_cost
### END FIXED MATERIAL COST ##########################################

def sum_one_timers(dictionary):
	total = 0.0
	for key in dictionary:
		if key.startswith("fixed-"):
			total += dictionary[key]
	return total

def awkit(dictionary):
	for key in sorted(dictionary.keys()):
		print(key, dictionary[key])

awkit(cost)
print("# TOTALS #################")
print("Total cost", sum(cost.values()))
print("Yearly cost", sum(cost.values()) - sum_one_timers(cost))
print("One-time cost", sum_one_timers(cost))
print("Land area", total_area)
