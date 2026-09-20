# Verification notes: chestnut, mycorrhiza, pecan

Checked against web sources on 2026-09-20. Deck and main research doc were not edited. "Deck says" = current wording.

## 1. Chinese vs European chestnut nuts
**Deck says:** Chinese: easy peeling, blight-resistant, ink disease "more resistant"; European: "often harder", susceptible, "very susceptible". Research doc also says mollissima is "sweet and waxy".

- **Peeling: supported, with a caveat.** An Ohio DNR page says the pellicle peels more readily from Chinese chestnuts than from European or Japanese ones, but the Finger Lakes Nut Farm article ("The Difference Between Chinese Chestnuts & European Chestnuts") and the Ohio page also note that many cultivars of both species have an easy-peel pellicle. So "often harder" for European is fair; "European = hard to peel" would not be.
- **Taste: partly supported.** Ohio DNR summary: Chinese chestnuts are less sweet/flavourful than American but much better tasting than European or Japanese. Consider "sweeter" hedged as "generally described as sweeter and better flavoured than European".
- **Size: the deck's "European often larger" is not supported.** The sources found say Chinese nuts are similar in size to typical European ones (about one inch, 30–50 per pound). Suggest dropping the size claim (it is only in speaker notes/research doc, not on the slide table).
- **Nutmeat colour:** Chinese and other oriental chestnuts have yellow nutmeat; American and European are white. Not in the deck; could be a small extra.
- **Blight: supported.** Chinese chestnut is resistant to *Cryphonectria* blight; the source says it is the most resistant of the north temperate species and that the European chestnut is susceptible (Ohio DNR; Arnold Arboretum article "Castanea mollissima: A Chinese Chestnut for the Northeast").
- **Ink disease: supported.** Academic sources (PLOS ONE linkage-map paper, PMC5589223; ISHS 494_55) say *C. sativa* has little to no resistance to *Phytophthora cinnamomi*, while the Asian species (*C. crenata*, *C. mollissima*) show the highest resistance. "Very susceptible" for sativa is fair; note *P. cambivora* also causes ink disease in Europe.

Sources: https://ohiodnr.gov/discover-and-learn/plants-trees/broad-leaf-trees/Chinese-chestnut-Castanea-mollissima · https://fingerlakesnutfarm.com/the-difference-between-chinese-chestnuts-european-chestnuts/ · https://arboretum.harvard.edu/arnoldia-stories/castanea-mollissima-a-chinese-chestnut-for-the-northeast/ · https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5589223/ · https://ishs.org/ishs-article/494_55/

## 2. Hardiness, ink disease and drainage for Ghent / the north garden
- **Hardiness:** Ohio DNR calls it cold-hardy across eastern North America (zones 5–6, marginal in Ohio); a German nursery search result said down to about -25 °C. Ghent's coldest night in each of 2015–2025 was between -2.6 and -8.3 °C (Open-Meteo, from the pecan analysis), so cold is not the constraint.
- **Drainage is the real risk.** A search summary (source pages listed below) states *C. mollissima* is sensitive to waterlogging and performs poorly in poorly drained soil, and ink-disease expression is worst with dry summer spells followed by frequent waterlogging in wet seasons. Resistance to the pathogen does not make it tolerant of standing water. `species.toml` already says "very sensitive to poor drainage". Because the north garden's soil is unknown (site under construction, regraded), **the site needs a drainage/soil check before planting**; the deck says nothing about this, and the research doc mentions drainage only briefly.
- Soil pH: `species.toml` gives 5.5–6.5; the Eetbaargoed listing pasted in `kastanje_wensen_paste.tsv` says acid to neutral, no lime. Fresh building-site soil often has lime/rubble, so pH is worth testing too (not verified from a source in this pass).

Sources: https://link.springer.com/article/10.1007/s13595-019-0898-8 · https://doi.org/10.3390/app152111695 · https://bsppjournals.onlinelibrary.wiley.com/doi/abs/10.1111/ppa.12313 · https://ohiodnr.gov/discover-and-learn/plants-trees/broad-leaf-trees/Chinese-chestnut-Castanea-mollissima

## 3. Reports from Flanders / NL / Germany
- **Little found.** Dutch-language results: Arborealis and Eetbaargoed sell *C. mollissima*; Kwekerij Culinair and De Bomelaar (Wetteren, currently out of stock) too. A COGEM (Dutch GMO committee) report exists on its dispersal biology, and Hortus Leiden lists the species.
- A Dutch source says the tree "does not occur naturally in the Netherlands but has been planted in some places", stays about 10–15 m in our climate, fruits around October, and that mollissima seedlings resemble the parents and "usually exceed them in production", so grafting is unnecessary for eating nuts (from the Arborealis catalogue text, a vendor claim).
- **No independent yield or performance reports for Grimo / Hebei North / SemRu seedlings in Belgium, NL or Germany were found.** Grimo is described as a Canadian-origin Chinese-type chestnut selected for disease resistance and nut size (a vendor description). Treat regional performance as unproven.
- Agroforestry Vlaanderen has an article on *C. sativa* (not mollissima): https://www.agroforestryvlaanderen.be/nl/nieuws/tamme-kastanje — possibly useful for local contacts.

Sources: https://www.arborealis.nl/catalog/bomen-en-heesters/castanea-mollisima.camollis.html · https://cogem.net/publicatie/vaststellen-verspreidingsbiologie-castanea-mollissima-chinese-tamme-kastanje/ · https://hortusleiden.gardenexplorer.org/taxon-22094.aspx · https://www.eetbaargoed.nl/product/tamme-kastanje-castanea-grimo-serie/

## 4. King Bolete (*Boletus edulis*) with chestnut
**Deck says:** chestnuts "partner with porcini fungi"; pre-inoculated trees exist; research doc says 15–40 kg/ha in managed Spanish groves and 5–10 years to fruiting.

- **Host compatibility with *C. mollissima* specifically: not confirmed in the sources found.** The evidence found is for European chestnut (*C. sativa*): Meotto et al. (1999, ISHS) got *B. edulis* mycorrhization on *C. sativa* using mycelial cultures, and it persisted in the field for 6 years; a PubMed paper (17359260) reports porcini-producing *C. sativa* forest. A 2021 *Forests* paper studied another ectomycorrhizal fungus on *C. henryi*.
- **Success rates:** in the field study, *B. edulis* mycorrhizae were on 82% of inoculated plants after the first period, **but no fruiting bodies were observed** while competing fungi fruited abundantly. Review text says artificial fruiting of *B. edulis* has not been achieved; spore inoculation was unsuccessful in the cited work, vegetative (mycelial) inoculum worked better.
- **So the deck's "5–10 years to mushrooms" and "double harvest" are optimistic.** The evidence supports mycorrhizal colonisation, not reliable fruiting. Suggest wording like "may fruit; not guaranteed" and dropping the 5–10 year figure unless a source is found.
- I could not verify the "15–40 kg/ha in Spain" figure in this pass (it came from earlier research; earlier deck text said up to 40 kg/ha "in Spain and China").

Sources: https://ishs.org/ishs-article/494_30/ · https://pubmed.ncbi.nlm.nih.gov/17359260/ · https://doi.org/10.3390/f12121643 · http://link.springer.com/10.1007/BF02861294

## 5. Pecan in Ghent (research doc section 3.5)
- **German reports agree with the deck's conclusion, and are more pessimistic.** A forum thread on garten-pur.de ("Pekan-Anbauversuche") and related German pages say there are pecan trees in Germany that have never produced a ripe nut, and that the nuts fall unripe after the November frosts because a European summer does not give enough season/heat. (Forum statement, anecdotal.)
- **Cultivars for short seasons:** German/Austrian nurseries offer 'Kanza' (ripens mid-October, described as early, disease-resistant) and 'Pawnee' (early, often early October). The northern-pecan blog's rule (about 950 cooling degree days, 180 frost-free days) still applies.
- **Commercial trials:** a German grower in the southern Palatinate has about 2 ha of pecans (FreshPlaza), testing fruit-fly-resistant types; no yield data given. A Dutch nut specialist reportedly grows pecans, no details found. **No confirmed report of pecans ripening in Belgium, the Netherlands or northern Germany was found.**
- **Conclusion:** the research doc's verdict stands (survives winters; ripening unreliable). It should say "no confirmed reports found" rather than "cannot ripen", and note that the 2026 heat (about 580 °F-days) is still well below the 950 threshold. If the local orchardist is real, ask for cultivar and whether the shucks split and nuts fill.

Sources: https://forum.garten-pur.de/viewtopic.php?t=1174&start=210 · https://schreiber-baum.at/produkt/kanza-pekannuss/ · https://www.baumschule-horstmann.de/pekannuss-pawnee-728_128050.html · https://www.freshplaza.com/europe/article/9832276/germany-shifts-to-nut-cultivation-as-climate-conditions-change/ · https://northernpecans.blogspot.com/2012/09/northern-pecans-climatic-adapation.html

## 6. Photos
See `image_sources.md`.
