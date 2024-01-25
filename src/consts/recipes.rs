pub const DEFAULT_PATH: &str = "recipes.yaml";

pub const DEFAULT_RECIPES: &str = "\
cocktails:
  - name: Martini
    ingredients:
      - gin
      - dry vermouth
    instructions: 50ml gin, 10ml vermouth, with an optional dash of bitters. Stir with ice and strain into glass. Garnish with a lemon twist.
  - name: Manhattan
    ingredients:
      - bourbon
      - sweet vermouth
      - angostura bitters
    instructions: 50ml bourbon, 20ml vermouth, dash of bitters. Stir with ice and strain into glass. Garnish with an orange twist.
  - name: Perfect Manhattan
    ingredients:
      - bourbon
      - sweet vermouth
      - dry vermouth
      - angostura bitters
    instructions: 50ml bourbon, 10ml sweet vermouth, 10ml dry vermouth, dash of bitters. Stir with ice and strain into glass. Garnish with an orange twist.
  - name: Dry Manhattan
    ingredients:
      - bourbon
      - dry vermouth
      - angostura bitters
    instructions: 50ml bourbon, 20ml vermouth, dash of bitters. Stir with ice and strain into glass. Garnish with an orange twist.
  - name: Americano
    ingredients:
      - campari
      - sweet vermouth
      - club soda
    instructions: 25ml campari, 25ml vermouth, 50ml club soda. Pour into a glass and mix up and down. Garnish with a lemon slice.
  - name: Negroni
    ingredients:
      - gin
      - campari
      - sweet vermouth
    instructions: 25ml of each. Pour into a glass with ice and stir. Garnish with a lemon or orange zest twist.
  - name: Old Fashioned
    ingredients:
      - bourbon
      - golden syrup
      - angostura bitters
    instructions: 50ml bourbon, 5ml syrup, dash of bitters. Pour in half of the bourbon with the syrup, bitters and ice into a glass with ice. Stir to dilute and then add the rest of the bourbon.
  - name: Sour
    ingredients:
      - bourbon
      - lemon juice
      - golden syrup
    instructions: 50ml bourbon, 15ml lemon juice, 10ml syrup. Shake with ice and strain into a glass with ice. Garnish with a cherry and lemon slice.
  - name: Boston Sour
    ingredients:
      - whiskey
      - lemon juice
      - golden syrup
    instructions: 50ml whiskey, 15ml lemon juice, 10ml syrup. Shake with ice and strain into a glass with ice. Garnish with a cherry and lemon slice.
  - name: Gin Sour
    ingredients:
      - gin
      - lemon juice
      - golden syrup
    instructions: 50ml gin, 15ml lemon juice, 10ml syrup. Shake with ice and strain into a glass with ice. Garnish with a cherry and lemon slice.
  - name: Green Park
    ingredients:
      - gin
      - lemon juice
      - golden syrup
      - basil leaves
      - egg white
    instructions: 50ml gin, 15ml lemon juice, 10ml syrup, 6 basil leaves, 15ml egg white. Shake everything with ice, strain out and remove the ice from the shaker. Return the liquid to the shaker to froth the egg. Pour into a cold glass. Garnish with a basil leaf.
  - name: Clover Club
    ingredients:
      - gin
      - lemon juice
      - golden syrup
      - dry vermouth
      - raspberries
      - egg white
    instructions: 50ml gin, 15ml lemon juice, 10ml syrup, 15ml dry vermouth, three raspberries, 15ml egg white. Shake everything with ice, strain out and remove the ice from the shaker. Return the liquid to the shaker to froth the egg. Pour into a cold glass. Garnish with a lemon twist or raspberry.
  - name: Mint Julep
    ingredients:
      - bourbon
      - mint leaves
      - golden syrup
    instructions: 50ml bourbon, 10 mint leaves, 5ml golden syrup. Bruise the mint leaves into the bottom of the glass. Add bourbon and syrup. Optionally add 20ml peach liqueur. Add crushed ice and stir. Garnish with mint.
  - name: French 75
    ingredients:
      - gin
      - lemon juice
      - golden syrup
      - champagne
    instructions: 30ml gin, 10ml lemon juice, 5ml syrup. Shake with ice and strain into a frozen champagne flute, top with champagne. Garnish with a lemon twist.
  - name: Tom Collins
    ingredients:
      - gin
      - lemon juice
      - golden syrup
      - club soda
    instructions: 50ml gin, 25ml lemon juice, 15ml syrup, 50ml club soda. Add all the ingredients to a glass with ice and stir. Garnish with a lemon slice.
  - name: Gin Fiz
    ingredients:
      - gin
      - lemon juice
      - golden syrup
      - club soda
    instructions: 50ml gin, 25ml lemon juice, 15ml syrup, 50ml club soda. Shake the gin, lemon juice, and syrup with ice. Strain into a glass with ice. Add the club soda on top stir. Garnish with a lemon slice.
  - name: Spritz
    ingredients:
      - dry white wine
      - campari
      - club soda
    instructions: 100ml wine, 50ml campari, 25ml club soda. Pour into a wine glass with ice and stir. Garnish with a half lemon slice.
  - name: Bronx
    ingredients:
      - gin
      - sweet vermouth
      - dry vermouth
      - orange juice
    instructions: 40ml gin, 20ml sweet vermouth, 20ml dry vermouth, 10ml orange juice. Shake everything with ice and strain into a cold glass. Garnish with an orange twist.
  - name: Avenue B
    ingredients:
      - bourbon
      - passion fruit pulp
      - grenadine
      - orange blossom water
    instructions: 50ml bourbon, 20ml fresh passion fruit pulp, 5ml grenadine, dash of orange blossom water. Shake everything with ice and strain into a cold glass.
  - name: Gimlet
    ingredients:
      - gin
      - lime cordial
    instructions: 50ml gin, 25ml lime cordial. Stir both with ice until slightly diluted and strain into a cold glass. Garnish with a lime wedge.
  - name: Army & Navy
    ingredients:
      - gin
      - lemon juice
      - orgeat syrup
    instructions: 50ml gin, 15ml lemon juice, 10ml orgeat syrup. Shake everything with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Ramos Gin Fizz
    ingredients:
      - gin
      - egg whites
      - golden syrup
      - single cream
      - orange blossom water
      - vanilla essence
      - lemon juice
      - lime juice
    instructions: For 2 people. 100ml gin, 2 egg whites, 40ml syrup, 50ml single cream, drops of orange blossom water, drops of vanilla essence, 25ml lemon juice, 25ml lime juice. Place egg whites into a bowl with the syrup and whisk until the batter becomes stiff, you should be able to turn the bowl upside down. Whisk further as you pour in the gin, the cream, orange blossom water, vanilla, and citrus. Transfer to a shaker with ice, shake, and strain into a cold ice. Garnish with a lime wedge.
  - name: Gin & It
    ingredients:
      - gin
      - sweet vermouth
      - angostura bitters
    instructions: 50ml gin, 25ml sweet vermouth, dash of bitters. Stir in a glass with ice. Garnish with a lemon twist.
  - name: Gin & Tonic
    ingredients:
      - gin
      - lime juice
      - tonic water
    instructions: 50ml gin, squeeze of lime juice, 100ml tonic water. Fill a glass with ice and pour in the gin. Squeeze over the lime and drop in the shell. Top with tonic water and stir.
  - name: Pin
    ingredients:
      - gin
      - lemon juice
      - golden syrup
      - pineapple juice
      - club soda
      - angostura bitters
    instructions: 50ml gin, 15ml lemon juice, 10ml syrup, 50ml pineapple juice. Pour the gin, lemon juice, syrup, and pineapple juice into a shaker with ice. Shake until frothy. Strain into a tall glass filled with ice. Top with club soda and stir. Add a few dashes of bitters onto the head as garnish.
  - name: English Breakfast Martini
    ingredients:
      - gin
      - lemon juice
      - marmalade
      - egg white
    instructions: 50ml tea-infused gin, 15ml lemon juice, 15ml marmalade, 15ml egg white. Shake all ingredients hard without ice and then again with ice. Strain into a cold glass.
  - name: Bacon Old Fashioned
    ingredients:
      - bourbon
      - maple syrup
      - angostura bitters
    instructions: 50ml bacon-infused bourbon, 10ml maple syrup. Pour the bourbon into a glass. Add the syrup and stir until dissolved. Add ice and stir more for dilution. Dash bitters and garnish with an orange twist.
  - name: Angostura Sour
    ingredients:
      - angostura bitters
      - lime juice
      - golden syrup
      - egg white
    instructions: 50ml bitters, 25ml lime juice, 25ml golden syrup, 15ml egg white. Shake hard with ice and strain into a cold glass.
  - name: Adonis
    ingredients:
      - sweet vermouth
      - fino sherry
      - orange bitters
    instructions: 30ml sweet vermouth, 30ml fino sherry, dash of orange bitters. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Affinity
    ingredients:
      - scotch
      - sweet vermouth
      - dry vermouth
      - peychaud's bitters
    instructions: 25ml scotch, 25ml sweet vermouth, 25ml dry vermouth, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a orange twist.
  - name: Alaska
    ingredients:
      - gin
      - yellow chartreuse
      - orange bitters
    instructions: 50ml gin, 15ml yellow chartreuse, dash orange bitters. Stir slowly over ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Algonquin
    ingredients:
      - rye
      - sweet white vermouth
      - pineapple juice
      - peychaud's bitters
    instructions: 50ml rye, 25ml sweet white vermouth, 25ml pineapple juice, dash of bitters. Stir in a glass with ice. Garnish with a cherry.
  - name: Ampersand
    ingredients:
      - gin
      - brandy
      - sweet vermouth
      - grand marnier
      - orange bitters
    instructions: 25ml old tom gin, 25ml brandy, 25ml sweet vermouth, 10ml grand marnier, dash orange bitters. Slowly stir over ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Arsenic and Old Lace
    ingredients:
      - gin
      - dry vermouth
      - crème de violette
      - absinthe
    instructions: 50ml gin, 20ml dry vermouth, 5ml crème de violette, 5ml absinthe. Slowly stir over ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Art of Choke
    ingredients:
      - light rum
      - mint leaves
      - cynar
      - green chartreuse
      - lime juice
      - golden syrup
    instructions: 25ml light rum, handful of mint leaves, 25ml cynar, 5ml green chartreuse, 2.5ml lime juice, 2.5ml syrup. Bruise the mint leaves in the bottom of the glass. Add all the other ingredients and stir. Garnish with a mint sprig.
  - name: B&B
    ingredients:
      - brandy
      - bénédictine
    instructions: 50ml brandy, 15ml bénédictine. Stir in a glass with ice. Garnish with a lemon twist.
  - name: Bamboo
    ingredients:
      - dry vermouth
      - coffee beans
      - fino sherry
      - grand marnier
    instructions: 30ml dry vermouth, 5 coffee beans, 30ml fino sherry, 5ml grand marnier. Leave the coffee beans in the vermouth to infuse for 15 minutes. Strain over ice with the other ingredients to stir. Strain into a cold glass.
  - name: BCC
    ingredients:
      - brandy
      - cassis
      - cloves
    instructions: 50ml brandy, 10ml cassis, 10 cloves. Muddle the cloves with the cassis. Strain into a glass with the brandy.
  - name: Bijou
    ingredients:
      - gin
      - sweet vermouth
      - green chartreuse
    instructions: 50ml gin, 20ml sweet vermouth, 5ml green chartreuse. Stir over large ice and strain into a cold glass. Garnish with an orang twist.
  - name: Bobby Burns
    ingredients:
      - scotch
      - sweet vermouth
      - drambuie
      - peychaud's bitters
    instructions: 50ml scotch, 25ml sweet vermouth, 5ml drambuie, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Boulevardier
    ingredients:
      - bourbon
      - sweet vermouth
      - campari
    instructions: 25ml bourbon, 25ml sweet vermouth, 25ml campari. Stir over ice in a glass. Garnish with lemon or orange slice.
  - name: Old Pal
    ingredients:
      - bourbon
      - dry vermouth
      - campari
    instructions: 25ml bourbon, 25ml dry vermouth, 25ml campari. Stir over ice in a glass. Garnish with lemon or orange slice.
  - name: Brooklyn
    ingredients:
      - rye
      - dry vermouth
      - amer picon
      - maraschino
    instructions: 45ml rye, 15ml dry vermouth, 10ml amer picon, 5ml maraschino. Stir over ice and strain into a cold glass. Garnish with a cherry.
  - name: Celery Cocktail
    ingredients:
      - gin
      - celery
      - lillet blanc
      - celery bitters
    instructions: 50ml gin, ten thin slices of celery, 10ml lillet blanc, dash of celery bitters. Muddle the celery in the bottom of the shaker with the gin and lillet. Stir over ice with bitters and strain into a cold glass. Garnish with a piece of celery.
  - name: Charco Di Sangre
    ingredients:
      - reposado tequila
      - grapefruit
      - amontillado sherry
      - crème de cassis
    instructions: 40ml tequila, 3 grapefruit zest twists, 20ml sherry, 20ml crème de cassis. Bash the twists in the bottom of the shaker before adding the alcohol. Stir with ice and strain into a cold glass.
  - name: Chocolate Martini
    ingredients:
      - vodka
      - mozart dry
      - cocchi americano
    instructions: 30ml vodka, 30ml mozart dry, 20ml cocchi americano. Stir slowly over large ice and strain into a cold glass. Garnish with an orange twist.
  - name: Chopin
    ingredients:
      - zubrowka
      - quinquina
      - orange bitters
    instructions: 50ml zubrowka, 25ml quinquina, dash of bitters. Stir in a glass with ice. Garnish with a lemon twist.
  - name: Chrysanthemum
    ingredients:
      - dry vermouth
      - bénédictine
      - absinthe
    instructions: 50ml dry vermouth, 10ml bénédictine, dash of absinthe. Stir slowly over large ice and strain into a cold glass. Garnish with an orange twist.
  - name: Consolation
    ingredients:
      - amontillado sherry
      - pisco
      - crème de pêche
      - farigoule de thym
    instructions: 30ml amontillado sherry, 10ml pisco, 10ml crème de pêche, 10ml farigoule de thym. Stir slowly over large ice and strain into a cold glass. Garnish with a lemon twist or sprig of thyme.
  - name: Corn N' Oil
    ingredients:
      - dark rum
      - falernum
      - angostura bitters
      - lime
    instructions: 50ml rum, 10ml falernum, dash of bitters, lime wedge. Stir the rum, falernum, and bitters together with ice in a glass. Squirt in some juice from the lime wedge and add in the shell.
  - name: \"Corpse Reviver #1\"
    ingredients:
      - brandy
      - apple brandy
      - sweet vermouth
    instructions: 40ml brandy, 20ml apple brandy, 20ml sweet vermouth. Stir slowly over large ice and strain into a cold glass.
  - name: Cucumber Cocktail
    ingredients:
      - gin
      - cucumber
      - golden syrup
    instructions: 50ml gin, 5cm cucumber, 10ml golden syrup. Peel the cucumber and muddle at the bottom of the shaker with the other ingredients. Stir with ice and strain into a glass. Garnish with a mint leaf.
  - name: Dunaway
    ingredients:
      - fino sherry
      - cynar
      - maraschino
      - angostura bitters
    instructions: 50ml fino sherry, 20ml cynar, 10ml maraschino, dash of bitters. Stir slowly over large ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Earl Grey Martini
    ingredients:
      - gin
      - earl grey tea leaves
      - dry vermouth
    instructions: 50ml gin, teaspoon of tea leaves, 10ml dry vermouth. Infuse the tea leaves in the gin for 1 minute and then strain into a shaker over ice. Stir in the vermouth. Strain into a glass. Garnish with a lemon slice.
  - name: East India
    ingredients:
      - brandy
      - grand marnier
      - maraschino
      - grenadine
      - angostura bitters
    instructions: 50ml brandy, 2.5ml grand marnier, 2.5ml maraschino, 5ml grenadine, dash of bitters. Stir slowly over large ice and strain into a cold glass. Garnish with an orange twist.
  - name: Edward I
    ingredients:
      - vodka
      - cynar
      - mozart dry
      - orange liqueur
    instructions: 40ml vodka, 20ml cynar, 5ml mozart dry, 5ml orange liqueur. Stir slowly over large ice and strain into a cold glass. Garnish with an orange twist.
  - name: Elderflower Martini
    ingredients:
      - gin
      - dry vermouth
      - elderflower liqueur
    instructions: 50ml gin, 25ml dry vermouth, 10ml elderflower liqueur. Stir slowly over large ice and strain into a cold glass. Garnish with a lemon twist.
  - name: El Presidente
    ingredients:
      - light rum
      - dry vermouth
      - grenadine
    instructions: 50ml light rum, 20ml dry vermouth, 5ml grenadine. Stir slowly over large ice and strain into a cold glass. Add gran marnier for any needed sweetness. Garnish with an orange twist.
  - name: Eye-Opener
    ingredients:
      - brandy
      - absinthe
      - crème de menthe
    instructions: 20ml brandy, 20ml absinthe, 20ml crème de menthe. Shake ingredients, add ice, and continue to shake. Strain into a glass. Garnish with chilli powder.
  - name: Fernando
    ingredients:
      - fernet branca
      - sweet white vermouth
      - galliano l'autentico
    instructions: 25ml fernet branca, 35ml vermouth, 15ml galliano l'autentico. Stir for some time over ice and strain into a cold glass. Garnish with a bruised mint leaf.
  - name: Gin Cocktail
    ingredients:
      - old tom gin
      - ginger syrup
      - angostura bitters
    instructions: 50ml gin, 5ml syrup, dash of bitters. Combine in a room temperature glass without ice.
  - name: Gin & Pine
    ingredients:
      - gin
      - pine tree splints
    instructions: 1 quart of gin, 2 ounces of pine splints. Soak the splints in the gin for two hours. Serve over ice.
  - name: Grosvenor
    ingredients:
      - kamm & sons
      - punt e mes
      - scotch
    instructions: 50ml kamm & sons, 25ml punt e mes, 5ml islay whiskey. Stir over ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Hanky Panky
    ingredients:
      - gin
      - sweet vermouth
      - fernet branca
    instructions: 35ml gin, 35ml sweet vermouth, 5ml vernet-branca. Stir slowly over large ice and strain into a cold glass. Garnish with an orange twist.
  - name: Harry's Cocktail
    ingredients:
      - gin
      - sweet vermouth
      - mint
      - absinthe
    instructions: 50ml gin, 25ml sweet vermouth, sprig of mint, dash of absinthe. gently bruise the mint in the bottom of the shaker. Add the other ingredients and stir. Strain into a cold glass.
  - name: Improved Brandy Cocktail
    ingredients:
      - brandy
      - golden syrup
      - maraschino
      - absinthe
      - angostura bitters
    instructions: 50ml brandy, 5ml golden syrup, 5ml maraschino, dash of absinthe, dash of bitters. Stir over ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Japanese Cocktail
    ingredients:
      - brandy
      - orgeat syrup
      - angostura bitters
    instructions: 50ml brandy, 5ml orgeat syrup, dash of bitters. Stir slowly over large ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Japanese Martini
    ingredients:
      - vodka
      - dry saké
    instructions: 50ml vodka, 25ml saké. Stir for a while over ice for dilution and strain into a cold glass. Garnish with a lime twist.
  - name: Journalist
    ingredients:
      - gin
      - dry vermouth
      - sweet vermouth
      - orange liqueur
      - lemon juice
      - angostura bitters
    instructions: 40ml gin, 10ml dry vermouth, 10ml sweet vermouth, 5ml orange liqueur, 5ml lemon juice, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a cherry.
  - name: Kangaroo
    ingredients:
      - vodka
      - dry vermouth
    instructions: 50ml vodka, 10ml dry vermouth. Stir with plenty of ice for at least 30 seconds and strain into a cold glass. Garnish with a lemon twist.
  - name: London Cocktail
    ingredients:
      - gin
      - golden syrup
      - absinthe
      - orange bitters
    instructions: 50ml gin, 5ml syrup, 5ml absinthe, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Louisiana
    ingredients:
      - rye
      - sweet vermouth
      - bénédictine
      - absinthe
      - pychaud's bitters
    instructions: 25ml rye, 25ml sweet vermouth, 25ml bénédictine, dash of absinthe, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a cherry.
  - name: Lucien Gaudin
    ingredients:
      - gin
      - campari
      - orange liqueur
      - dry vermouth
    instructions: 40ml gin, 20ml campari, 20ml orange liqueur, 20ml dry vermouth. Stir with ice and strain into a cold glass. Garnish with a cherry.
  - name: Martiki
    ingredients:
      - light rum
      - coconut water
      - kummel
    instructions: 50ml light rum, 25ml coconut water, 10ml kummel. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Martinez
    ingredients:
      - gin
      - sweet vermouth
      - maraschino
      - angostura bitters
    instructions: 25ml gin, 50ml sweet vermouth, 5ml maraschino, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a cherry.
  - name: Metropolitan
    ingredients:
      - brandy
      - sweet vermouth
      - angostura bitters
    instructions: 50ml brandy, 25ml sweet vermouth, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Moonraker
    ingredients:
      - brandy
      - quinquina
      - crème de pêche
      - absinthe
    instructions: 25ml brandy, 25ml quinquina, 25ml crème de pêche, absinthe. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Nymph
    ingredients:
      - rye
      - sweet white vermouth
      - apricot brandy
      - angostura bitters
    instructions: 25ml rye, 25ml sweet white vermouth, 25ml apricot brandy, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Oaxaca Old Fashioned
    ingredients:
      - reposado tequila
      - mezcal
      - agave syrup
      - angostura bitters
    instructions: 50ml tequila, 10ml mezcal, 2.5ml agave syrup, dash of bitters. Stir the tequila, syrup, and bitters without ice in a glass to dissolve the syrup. Add the mezcal with some ice and stir. Garnish with an orange twist.
  - name: Obituary
    ingredients:
      - gin
      - dry vermouth
      - absinthe
    instructions: 35ml gin, 35ml dry vermouth, dash of absinthe. Rinse the glass with the absinthe and leave to chill in the freezer. Stir the gin and dry vermouth over ice and strain into the glass. Garnish with a cherry.
  - name: Old Etonian
    ingredients:
      - gin
      - sweet white vermouth
      - crème de noyaux
      - orange bitters
    instructions: 35ml gin, 35ml sweet white vermouth, 2.5ml crème de noyaux, dash of bitters. Stir with ice and strain into a cold glass. Garnish with an orange twist.
  - name: Pablo Alvarez De Canas Special
    ingredients:
      - brandy
      - fino sherry
      - cherry brandy
      - lemon
    instructions: 30ml brandy, 30ml fino sherry, 10ml cherry brandy, 1 lemon twist. Muddle the lemon twist and then add the other ingredients. Stir with ice and strain into a cold glass. Garnish with a cherry.
  - name: Pacific Coast Highway
    ingredients:
      - tequila
      - yellow chartreuse
      - mezcal
      - grapefruit
    instructions: 50ml tequila, 5ml yellow chartreuse, 10ml mezcal, 1 grapefruit twist. Pour the chartreuse into a glass with ice and roll to rinse the glass and ice. Express the oils from the grapefruit twist and add to the glass. Pour in the rest of the ingredients and stir.
  - name: Ping-Pong
    ingredients:
      - sloe gin
      - dry vermouth
      - sweet vermouth
      - orange bitters
    instructions: 50ml sloe gin, 10ml dry vermouth, 10ml sweet vermouth, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Pink Gin
    ingredients:
      - gin
      - angostura bitters
    instructions: 50ml navy-strength gin, 2.5ml bitters. Add the bitters to a room temperature glass and roll to coat the glass. Pour in the gin and serve room temperature without ice.
  - name: Pink Rabbit
    ingredients:
      - pisco
      - sweet white vermouth
      - crème de rose
      - absinthe
    instructions: 50ml pisco, 25ml sweet white vermouth, 10ml crème de rose, dash of absinthe. Stir with ice and strain into a cold glass. Garnish with a rose petal.
  - name: Poet's Dream
    ingredients:
      - gin
      - dry vermouth
      - bénédictine
    instructions: 40ml gin, 20ml dry vermouth, 10ml bénédictine. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Referendum
    ingredients:
      - scotch
      - elderflower liqueur
      - peychaud's bitters
    instructions: 50ml scotch, 15ml elderflower liqueur, dash of bitters. Stir over ice in a glass. Garnish with a lemon twist.
  - name: Remember the Maine
    ingredients:
      - rye
      - sweet vermouth
      - cherry brandy
      - absinthe
    instructions: 50ml rye, 20ml sweet vermouth, 5ml cherry brandy, dash of absinthe. Stir with ice and strain into a cold glass. Garnish with a cherry.
  - name: Rob Roy
    ingredients:
      - scotch
      - sweet vermouth
      - peychaud's bitters
    instructions: 50ml scotch, 25ml sweet vermouth, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a cherry.
  - name: Rose
    ingredients:
      - gin
      - cherry brandy
      - dry vermouth
      - rose water
    instructions: 40ml gin, 20ml cherry brandy, 20ml dry vermouth, dash of rose water. String with ice and strain into a cold glass. Garnish with a rose petal.
  - name: Rosita
    ingredients:
      - reposado tequila
      - campari
      - sweet vermouth
    instructions: 50ml tequila, 15ml campari, 15ml sweet vermouth. Stir over ice in a glass and garnish with a lemon slice.
  - name: Rusty Nail
    ingredients:
      - scotch
      - drambuie
    instructions: 50ml scotch, 10ml drambuie. Stir over ice in a glass. Garnish with a lemon twist.
  - name: Saratoga
    ingredients:
      - brandy
      - rye
      - sweet vermouth
      - angostura bitters
    instructions: 25ml brandy, 25ml rye, 25ml sweet vermouth, dash of bitters. Stir with ice and strain into a chilled glass. Garnish with a lemon slice.
  - name: Sazerac
    ingredients:
      - bourbon
      - brandy
      - brown sugar
      - absinthe
      - peychaud's bitters
      - angostura bitters
    instructions: 30ml bourbon, 30ml brandy, 2.5ml brown sugar, dash of absinthe, dash of peychaud's bitters, dash of angostura bitters. Coat the surface of the glass with the absinthe and place into the freezer. Stir the rest of the ingredients in a shaker without ice to melt the sugar. Add ice and stir to chill. Discard any extra absinthe from the glass and strain the contents of the shaker into the glass. Garnish with a lemon twist.
  - name: Skid Row
    ingredients:
      - genever
      - apricot brandy
      - yellow chartreuse
      - angostura bitters
    instructions: 50ml genever, 15ml apricot brandy, 15ml yellow chartreuse, dash of bitters. Stir with ice and strain into a cold glass.
  - name: Spring Green
    ingredients:
      - fino sherry
      - green chartreuse
      - elderflower liqueur
    instructions: 45ml fino sherry, 15ml green chartreuse, 15ml elderflower liqueur. Stir with ice and strain into a cold glass.
  - name: Stinger
    ingredients:
      - brandy
      - crème de menthe
    instructions: 50ml brandy, 15ml crème de menthe. Stir over ice in a glass. Garnish with a mint leaf.
  - name: Gin Stinger
    ingredients:
      - gin
      - crème de menthe
    instructions: 50ml gin, 15ml crème de menthe. Stir over ice in a glass. Garnish with a mint leaf.
  - name: Fancy Stinger
    ingredients:
      - gin
      - crème de menthe
      - fernet branca
    instructions: 50ml brandy, 15ml crème de menthe, dash of fernet branca. Rinse the glass with the fernet branca. Stir in the other ingredients with ice. Garnish with a mint leaf.
  - name: Stomach Reviver
    ingredients:
      - brandy
      - kummel
      - fernet branca
      - angostura bitters
    instructions: 25ml brandy, 25ml kummel, 10ml fernet branca, 2.5ml bitters. Stir over ice and strain into a cold glass.
  - name: Suburban
    ingredients:
      - bourbon
      - dark rum
      - port
      - angostura bitters
      - orange bitters
    instructions: 40ml bourbon, 20ml dark run, 20ml port, dash of angostura bitters, dash of orange bitters. Stir with ice and strain into a cold glass. Garnish with an orange twist.
  - name: Sylvanian Martini
    ingredients:
      - gin
      - alpine liqueur
      - amontillado sherry
      - angostura bitters
    instructions: 50ml gin, 2.5ml alpine liqueur, 25ml amontillado sherry, dash of bitters. Rinse the glass with the alpine liqueur and place in the freezer. Stir the rest of the ingredients in a shaker with ice and strain into the glass.
  - name: Tipperary
    ingredients:
      - irish whiskey
      - sweet vermouth
      - green chartreuse
    instructions: 50ml whiskey, 25ml sweet vermouth, 5ml green chartreuse. Stir with ice and strain into a cold glass.
  - name: Trilby
    ingredients:
      - gin
      - sweet vermouth
      - crème de violette
      - angostura bitters
      - orange bitters
    instructions: 35ml old tom gin, 35ml sweet vermouth, 10ml crème de violette, dash of angostura bitters, dash of orange bitters. Stir with ice and strain into a cold glass. Garnish with a cherry.
  - name: Tuxedo
    ingredients:
      - gin
      - dry vermouth
      - fino sherry
      - maraschino
      - absinthe
      - orange bitters
    instructions: 25ml gin, 25ml dry vermouth, 25ml fino sherry, 2.5ml maraschino, dash of absinthe, dash of orange bitters. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Vieux Carré
    ingredients:
      - brandy
      - bourbon
      - sweet vermouth
      - bénédictine
      - peychaud's bitters
    instructions: 25ml brandy, 25ml bourbon, 25ml sweet vermouth, 5ml bénédictine, dash of bitters. Stir over large ice and strain into a cold glass.
  - name: Whisky Mac
    ingredients:
      - scotch
      - ginger wine
    instructions: 50ml scotch, 25ml ginger wine (or 15ml ginger liqueur). Stir over ice in a glass.
  - name: Widow's Kiss
    ingredients:
      - apple brandy
      - bénédictine
      - green chartreuse
      - angostura bitters
    instructions: 50ml apple brandy, 15ml bénédictine, 15ml green or yellow chartreuse, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a mint sprig.
  - name: Purgatory
    ingredients:
      - rye
      - bénédictine
      - green chartreuse
      - angostura bitters
    instructions: 50ml rye, 15ml bénédictine, 15ml green or yellow chartreuse, dash of bitters. Stir with ice and strain into a cold glass. Garnish with a mint sprig.
  - name: Absinthe Suissesse
    ingredients:
      - absinthe
      - mint
      - orgeat syrup
      - egg white
    instructions: 50ml absinthe, handful of mint leaves, 10ml orgeat syrup, 20ml egg white. Dry shake to froth the egg, then add ice and shake again. Strain into a cold glass. Garnish with a mint leaf.
  - name: Amaretto Sour
    ingredients:
      - bourbon
      - amaretto
      - lemon juice
      - golden syrup
      - egg white
    instructions: 25ml bourbon, 25ml amaretto, 15ml lemon juice, 5ml syrup, 10ml egg white. Dry shake to froth the egg, then add ice and shake again. Strain into a cold glass. Garnish with a bitters.
  - name: Aperol Sour
    ingredients:
      - gin
      - aperol
      - lemon juice
      - golden syrup
      - egg white
    instructions: 25ml gin, 25ml aperol, 15ml lemon juice, 5ml syrup, 10ml egg white. Dry shake to froth the egg, then add ice and shake again. Strain into a cold glass. Garnish with a bitters.
  - name: Aviation
    ingredients:
      - gin
      - lemon juice
      - maraschino
    instructions: 50ml gin, 15ml lemon juice, 10ml maraschino. Optionally add 2.5ml violet liqueur. Shake with ice and strain into a cold glass. Garnish with a cherry.
  - name: Eagle's Dream
    ingredients:
      - gin
      - lemon juice
      - maraschino
      - egg white
    instructions: 50ml gin, 15ml lemon juice, 10ml maraschino, 15ml egg white. Optionally add 2.5ml violet liqueur. Dry shake to froth the egg, then add ice and shake again. Strain into a cold glass. Garnish with a cherry.
  - name: Casino
    ingredients:
      - gin
      - lemon juice
      - maraschino
      - orange juice
    instructions: 50ml gin, 15ml lemon juice, 10ml maraschino, 20ml orange juice. Optionally add 2.5ml violet liqueur. Shake with ice and strain into a cold glass. Garnish with a cherry.
  - name: King's Jubilee
    ingredients:
      - light rum
      - lemon juice
      - maraschino
    instructions: 50ml light rum, 15ml lemon juice, 10ml maraschino. Optionally add 2.5ml violet liqueur. Shake with ice and strain into a cold glass. Garnish with a cherry.
  - name: Bacardi Special
    ingredients:
      - light rum
      - gin
      - lime juice
      - grenadine
    instructions: 35ml light rum, 15ml gin, 10ml lime juice, 5ml grenadine. Shake with ice and strain into a cold glass. Garnish with a lime wedge.
  - name: Simple Bacardi
    ingredients:
      - light rum
      - lime juice
      - grenadine
    instructions: 50ml light rum, 10ml lime juice, 5ml grenadine. Shake with ice and strain into a cold glass. Garnish with a lime wedge.
  - name: Barbara West
    ingredients:
      - gin
      - amontillado sherry
      - lemon juice
      - golden syrup
      - angostura bitters
    instructions: 25ml gin, 25ml sherry, 15ml lemon juice, 10ml syrup, dash of bitters. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Barnum Was Right
    ingredients:
      - gin
      - apricot brandy
      - lemon juice
      - angostura bitters
    instructions: 50ml gin, 25ml brandy, 15ml lemon juice, dash of bitters. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Batida
    ingredients:
      - cachaça
      - mango
      - golden syrup
      - lime juice
    instructions: 50ml cachaça, 1/2 fresh mango, 5ml syrup, squirt of lime juice. Skin the mango and cut up the fuit. Put the fruit pieces into the shaker and muddle with the cachaça. Add the rest of the ingredients. Shake with ice and strain into a cold glass. Garnish with a lime wedge.
  - name: Bee's Knees
    ingredients:
      - gin
      - lemon juice
      - honey
    instructions: 50ml gin, 15ml lemon juice, 10ml honey loosened with 5ml of hot water. Add ingredients to the shaker and make sure to disolve the honey. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Between the Sheets
    ingredients:
      - brandy
      - light rum
      - orange liqueur
      - lemon juice
    instructions: 25ml brandy, 25ml light rum, 25ml orange liqueur, 10ml lemon juice. Shake with ice for longer than normal and strain into a cold glass. Garnish with a lemon twist.
  - name: Blood & Sand
    ingredients:
      - scotch
      - cherry brandy
      - sweet vermouth
      - orange juice
    instructions: 40ml scotch, 20ml cherry brandy, 20ml sweet vermouth, 20ml (blood) orange juice. Shake with ice and strain into a cold glass. Garnish with an orange twist.
  - name: Branble
    ingredients:
      - gin
      - lemon juice
      - golden syrup
      - crème de mure
    instructions: 50ml gin, 25ml lemon juice, 10ml syrup, 10ml crème de mure. Shake the gin, lemon, and syrup with ice and strain into a glass with crushed ice. Pour the liqueur on top without mixing to create a bleeding effect. Garnish with mint, lemon, or blackberries. Serve with a straw.
  - name: Brandy Alexander
    ingredients:
      - brandy
      - crème de cacao
      - single cream
      - nutmeg
    instructions: 50ml brandy, 25ml crème de cacao, 25ml single cream, grated nutmeg. Shake everything but the nutmeg with ice and strain into a cold glass. Grate the nutmeg on top.
  - name: Brandy Fix
    ingredients:
      - brandy
      - pineapple syrup
      - lime juice
      - green chartreuse
    instructions: 50ml brandy, 15ml syrup, 15ml lime juice, 5ml green chartreuse. Shake with ice and strain into a glass with ice. Garnish with pineapple.
  - name: Brown Derby
    ingredients:
      - bourbon
      - pink grapefruit juice
      - maple syrup
    instructions: 45ml bourbon, 30ml pink grapefruit juice, 10ml maple syrup. Shake with ice and strain into a cold glass. Garnish with a grapefruit twist.
  - name: Caipirinha
    ingredients:
      - cachaça
      - lime
      - golden caster sugar
    instructions: 50ml cachaça, 1/2 lime, 2.5ml sugar. Slice the lime into three wedges and place in the bottom of a sturdy glass with the sugar. Muddle hard until all the sugar is completely dissolved. Add the cachaça, fill the rest of the glass with ice, and stir. Garnish with a lime wedge.
  - name: Campari Smash
    ingredients:
      - vodka
      - campari
      - crème de rose
      - strawberries
      - mint leaves
    instructions: 40ml vodka, 20ml campari, 20ml crème de rose, 2 fresh strawberries, 5 mint leaves. Muddle the berries and mint in the bottom of the shaker. Add the rest of the ingredients and shake with ice. Strain into a cold glass. Garnish with mint.
  - name: Champs-Elysées
    ingredients:
      - brandy
      - lemon juice
      - green chartreuse
      - golden syrup
      - angostura bitters
    instructions: 50ml brandy, 15ml lemon juice, 10ml green chartreuse, 5ml syrup, dash of bitters. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Chanticleer
    ingredients:
      - gin
      - dry vermouth
      - orange liqueur
      - egg white
    instructions: 30ml gin
  - name: Charlie Chaplin
    ingredients:
      - gin
      - sloe gin
      - apricot brandy
      - lime juice
    instructions: 40ml gin, 20ml sloe gin, 20ml apricot brandy, 20ml lime juice. Shake with ice and strain into a cold glass. Garnish with a lime twist.
  - name: Cherry Bomb
    ingredients:
      - bourbon
      - cherry brandy
      - lemon juice
      - golden syrup
      - egg white
    instructions: 50ml brandy, 20ml cherry brandy, 20ml lemon juice, 10ml syrup, 20ml egg white. Shake hard without ice and then again with ice. Strain into a cold glass with ice. Garnish with a cherry.
  - name: \"Corpse Reviver #2\"
    ingredients:
      - gin
      - orange liqueur
      - lillet blanc
      - lemon juice
      - absinthe
    instructions: 20ml gin, 20ml orange liqueur, 20ml lillet blanc, 20ml lemon juice, 2.5ml absinthe. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Cosmopolitan
    ingredients:
      - lemon vodka
      - orange liqueur
      - cranberry juice
      - lime juice
    instructions: 45ml vodka, 15ml orange liqueur, 30ml cranberry juice, 7.5ml lime juice. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Cotonian
    ingredients:
      - scotch
      - drambuie
      - dry vermouth
      - passion fruit pulp
    instructions: 40ml scotch, 20ml drambuie, 20ml dry vermouth, 20ml passion fruit pulp. Shake with ice and strain into a cold glass. Garnish with a lime twist.
  - name: Cucumber Gimlet
    ingredients:
      - gin
      - cucumber
      - mint
      - lime cordial
    instructions: 50ml gin, 5cm cucumber, handful of mint leaves, 10ml lime cordial. Peel the cucumber and chop into pieces. Place in the shaker with the mint and muddle. Add the rest and shake with ice, strain into a cold glass. Garnish with a cucumber slice.
  - name: Daiquiri
    ingredients:
      - light rum
      - lime juice
      - golden syrup
    instructions: 50ml rum, 15ml lime juice, 10ml syrup. Shake with ice and strain into a cold glass. Garnish with a lime wedge.
  - name: Miami
    ingredients:
      - light rum
      - lime juice
      - golden syrup
      - mint leaves
    instructions: 50ml rum, 15ml lime juice, 10ml syrup, handful of mint leaves. Shake with ice and strain into a cold glass. Garnish with a lime wedge.
  - name: Diki-Diki
    ingredients:
      - apple brandy
      - swedish punsch
      - pink grapefruit juice
    instructions: 50ml apple brandy, 25ml swedish punsch, 25ml pink grapefruit juice. Shake with ice and strain into a cold glass. Garnish with a grapefruit wedge.
  - name: Dulchin
    ingredients:
      - pisco
      - grand marnier
      - apricot brandy
      - lime juice
      - greadine
    instructions: 50ml pisco, 10ml grand marnier, 10ml apricot brandy, 10ml lime juice, 5ml grenadine. Shake with ice and strain into a cold glass. Garnish with a lime wedge.
  - name: Espresso Martini
    ingredients:
      - vodka
      - espresso
      - sugar
    instructions: 50ml vodka, 1 shot of espress, teaspoon of sugar. Brew the espresso and stir in the sugar until it's dissolved. Add extra ice to the shaker and pour the ingredients over. Shake hard and strain into a cold glass.
  - name: Figaro
    ingredients:
      - fino sherry
      - black fig
      - lemon juice
      - golden syrup
    instructions: 50ml fino sherry, 1 black fig, 15ml lemon juice, 10ml syrup. Cut the fig into 8 pieces and place 7 of them into the shaker with the other ingredients. Shake with ice and remove the fig husks and seeds. Decant the liquid into a cold glass and garnish with the last piece of fig.
  - name: Flip
    ingredients:
      - whiskey
      - egg yolk
      - golden syrup
      - vanilla extract
    instructions: 50ml whiskey, 1 egg yolk, 10ml golden syrup, dash of vanilla extract. SHake with ice and strain into a glass with ice. Grate nutmeg or cinnamon to garnish.
  - name: Florida
    ingredients:
      - light rum
      - lime juice
      - sweet vermouth
      - white crème de cacao
      - orange liqueur
      - grenadine
    instructions: 45ml light rum, 20ml lime juice, 15ml sweet vermouth, 5ml white crème de cacao, 2.5ml orange liqueur, 2.5ml grenadine. Shake with ice and strain into a cold glass. Garnish with an orange twist.
  - name: French Martini
    ingredients:
      - vodka
      - pineapple juice
      - chambord
    instructions: 40ml vodka, 30ml pineapple juice, 5ml chambord. Shake with ice until the juice is frothed. Strain into a cold glass. Garnish with a pineapple wedge.
  - name: Ginger Alexander
    ingredients:
      - bourbon
      - ginger liqueur
      - crème de cacao
      - single cream
    instructions: 25ml bourbon, 25ml ginger liqueur, 25ml crème de cacao, 25ml single cream. Shake with ice and strain into a cold glass. Garnish with a dusting of ginger powder.
  - name: Golden Dawn
    ingredients:
      - gin
      - calvados
      - apricot brandy
      - orange juice
      - grenadine
    instructions: 20ml gin, 20ml calvados, 20ml apricot brandy, 20ml orange juice, 5ml grenadine. Shake everything but the grenadine with ice and strain into a cold glass. Trickle the grenadine over without mixing.
  - name: Grasshopper
    ingredients:
      - crème de cacao
      - crème de menthe
      - single cream
    instructions: 25ml crème de cacao, 25ml crème de menthe, 25ml single cream. SHake with ice and strain into a cold glass. Garnish with a mint leaf.
  - name: Hemingway Daiquiri
    ingredients:
      - light rum
      - lime juice
      - maraschino
      - grapefruit juice
    instructions: 50ml light rum, 15ml lime juice, 10ml maraschino, 10ml grapefruit juice. SHake with ice and strain into a cold glass. Garnish with a lime wedge.
  - name: Holland House
    ingredients:
      - gin
      - dry vermouth
      - maraschino
      - lemon juice
      - pineapple juice
    instructions: 40ml gin, 20ml dry vermouth, 5ml maraschino, 10ml lemon juice, 10ml pineapple juice. SHake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Ideal
    ingredients:
      - gin
      - sweet vermouth
      - grapefruit juice
      - maraschino
    instructions: 50ml gin, 25ml sweet vermouth, 10ml grapefruit juice, 5ml maraschino. Shake with ice and strain into a cold glass. Garnish with a lime twist.
  - name: Jack Rose
    ingredients:
      - apple brandy
      - lemon juice
      - grenadine
    instructions: 50ml apple brandy, 15ml lemon juice, 10ml grenadine. Shake with ice and strain into a cold glass. Garnish with an apple slice moistened with lemon juice.
  - name: Kingston Cocktail
    ingredients:
      - dark rum
      - kummel
      - orange juice
      - pimento dram
    instructions: 50ml dark rum, 15ml kummel, 15ml orange juice, 2.5ml pimento dram. Shake with ice and strain into a cold glass.
  - name: Knickerbocker
    ingredients:
      - dark rum
      - lemon juice
      - orange liqueur
      - golden syrup
      - raspberries
    instructions: 50ml dark rum, 15ml lemon juice, 10ml orange liqueur, 5ml syrup, two fresh raspberries. Shake with ice until the raspberries are blended and strain into a glass with ice. Garnish with raspberries.
  - name: Last Word
    ingredients:
      - gin
      - green chartreuse
      - maraschino
      - lime juice
    instructions: 40ml gin, 20ml green chartreuse, 20ml maraschino, 20ml lime juice. Shake with ice and strain into a cold glass. Garnish with a lime wedge.
  - name: Mandorla Sour
    ingredients:
      - grappa
      - lemon juice
      - golden syrup
      - egg white
    instructions: 50ml Mandorla Grappa by Nardini, 25ml lemon juice, 15ml golden syrup, 10ml egg white. Dry shake to froth the egg, then shake with ice. Strain into a glass with ice. Garnish with a cherry and the juice from the jar.
  - name: Margarita
    ingredients:
      - tequila
      - orange liqueur
      - lime juice
    instructions: 50ml tequila, 20ml orange liqueur, 15ml lime juice. Cut a wedge of lime and wipe around the edge of the glass. Dip the glass rim into a plate of flaked sea salt. Shake the ingredients with ice and strain into the salted glass. Garnish with a lime wedge.
  - name: Mary Pickford
    ingredients:
      - light rum
      - pineapple juice
      - grenadine
      - maraschino
    instructions: 40ml light rum, 40ml pineapple juice, 5ml grenadine, dash of maraschino. Shake with ice and strain into a cold glass. Garnish with a cherry.
  - name: Mexican Jumping Bean
    ingredients:
      - tequila reposado
      - espresso
      - agave syrup
    instructions: 50ml tequila, 25ml espresso, 10ml syrup. Shake with extra ice and strain into a chilled glass.
  - name: Monkey Gland
    ingredients:
      - gin
      - orange juice
      - absinthe
      - grenadine
    instructions: 50ml gin, 30ml orange juice, 5ml absinthe, 5ml grenadine. Shake with ice and strain into a cold glass. Garnish with an orange twist.
  - name: Naked and Famous
    ingredients:
      - mezcal
      - yellow chartreuse
      - aperol
      - lime juice
    instructions: 20ml mezcal, 20ml yellow chartreuse, 20ml aperol, 20ml lime juice. Shake with ice and strain into a cold glass.
  - name: Nuclear Daiquiri
    ingredients:
      - light rum
      - lime juice
      - falernum
      - green chartreuse
    instructions: 50ml light rum, 15ml lime juice, 10ml falernum, 10ml green chartreuse. Shake with ice and strain into a cold glass. Garnish with a lime wedge.
  - name: Nursery Plush
    ingredients:
      - vodka
      - bénédictine
      - single cream
      - green cardamom pods
    instructions: 25ml vodka, 25ml bénédictine, 25ml single cream, 5-6 cardamom pods. Muddle the cardamom pods slightly. Add the rest of the ingredients and shake with ice. Strain into a cold glass.
  - name: Oh Gosh!
    ingredients:
      - light rum
      - orange liqueur
      - lime juice
    instructions: 50ml light rum, 20ml orange liqueur, 15ml lime juice. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: One Way
    ingredients:
      - 40ml gin
      - 20ml crème de peche
      - swedish punsch
      - lemon juice
    instructions: 40ml gin, 20ml crème de peche, 20ml swedish punsch, 20ml lemon juice. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Opal
    ingredients:
      - gin
      - orange liqueur
      - orange juice
      - orange blossom water
      - orange bitters
    instructions: 40ml gin, 10ml orange liqueur, 30ml orange juice, dash of orange blossom water, dash of orange bitters. Shake with ice and strain into a cold glass. Garnish with an orange twist.
  - name: Pan-American Clipper
    ingredients:
      - apple brandy
      - absinthe
      - lime juice
      - grenadine
    instructions: 50ml apple brandy, dash of absinthe, 15ml lime juice, 10ml grenadine. Rinse the glass with the absinthe and freeze. Shake the other ingredients with ice and strain into the glass.
  - name: Paper Plane
    ingredients:
      - bourbon
      - aperol
      - amaro nonino
      - lemon juice
    instructions: 40ml bourbon, 20ml aperol, 20ml amaro nonino, 20ml lemon juice. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Pegu Club
    ingredients:
      - gin
      - orange liqueur
      - lime juice
      - angostura bitters
      - orange bitters
    instructions: 50ml gin, 20ml orange liqueur, 15ml lime juice, dash of bitters, dash of orange bitters. Shake with ice and strain into a cold glass.
  - name: Penicillin
    ingredients:
      - scotch
      - ginger liqueur
      - lemon juice
      - honey
    instructions: 40ml blended scotch, 10ml smoky single malt scotch, 10ml ginger liqueur, 15ml lemon juice, 10ml honey loosened with hot water. Shake with ice and strain into a glass with a lump of ice. Garnish with a piece of ginger.
  - name: Pink Lady
    ingredients:
      - gin
      - apple brandy
      - lemon juice
      - grenadine
      - egg white
    instructions: 25ml gin, 25ml apple brandy, 15ml lemon juice, 10ml grenadine, 10ml egg white. Reverse double shake by first shaking hard with ice, strain into another vessel, empty the shaker, and return the cocktail to the shaker and shake hard to froth the egg. Pour into a cold glass. Garnish with a cherry.
  - name: Pisco Sour
    ingredients:
      - pisco
      - lime juice
      - golden syrup
      - egg white
      - angostura bitters
    instructions: 50ml pisco, 25ml lime juice, 15ml golden syrup, 10ml egg white, dash of bitters. Dry shake the pisco, lime juice, syrup, and egg white. Shake with ice and strain into a glass filled with ice. Garnish with a dash of the bitters.
  - name: Red Lion
    ingredients:
      - gin
      - grand marnier
      - lime juice
      - grenadine
    instructions: 50ml gin, 10ml Grand Marnier, 15ml lime juice, 5ml grenadine. Shake with ice and strain into a cold glass. Garnish with an orange twist
  - name: Sangre De Agava
    ingredients:
      - tequila reposado
      - dark rum
      - lime juice
      - crème de cassis
    instructions: 45ml tequila, 15ml dark rum, 20ml lime juice, 15ml crème de cassis. Shake with ice and strain into a cold glass.
  - name: Santiago
    ingredients:
      - light rum
      - lemon juice
      - grenadine
    instructions: 50ml light rum, 15ml lemon juice, 10ml grenadine. Shake with ice and strain into a cold glass. Garnish with a cherry.
  - name: Scofflaw
    ingredients:
      - bourbon
      - dry vermouth
      - lemon juice
      - grenadine
      - orange bitters
    instructions: 30ml bourbon, 30ml dry vermouth, 15ml lemon juice, 10ml grenadine, dash of orange bitters. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Sidecar
    ingredients:
      - brandy
      - orange liqueur
      - lemon juice
    instructions: 50ml brandy, 20ml orange liqueur, 15ml lemon juice. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Silver Bullet
    ingredients:
      - gin
      - kummel
      - lemon juice
      - egg white
    instructions: 50ml gin, 25ml kummel, 25ml lemon juice, 15ml egg white, optionally a dash of celery bitters. Shake with ice and strain into a glass. Garnish with a lemon twist.
  - name: Teresa
    ingredients:
      - vodka
      - campari
      - lime juice
      - crème de cassis
    instructions: 25ml vodka, 25ml campari, 25ml lime juice, 10ml crème de cassis. Shake with sice and strain into a cold glass. Garnish with a mint sprig.
  - name: Ti' Punch
    ingredients:
      - rhum agricole
      - lime
      - golden syrup
    instructions: 50ml rhum agricole, half a lime cut into wedges, 10ml syrup. Mudle everything in the bottom of a glass until well combined. Fill with ice and granish with a lime wedge.
  - name: Tommy's Margarita
    ingredients:
      - tequila
      - lime juice
      - agave syrup
      - sea salt
    instructions: 50ml tequila, 15ml lime juice, 10ml agave syrup. Rim half of a glass with the salt and fill with ice. Shake the other ingredients with ice and strain into the glass. Garnish with a lime wedge.
  - name: Toreador
    ingredients:
      - tequila
      - apricot brandy
      - lime juice
    instructions: 50ml tequila, 20ml apricot brandy, 20ml lime juice. Shake with ice and strain into a cold glass. Garnish with a lime wedge.
  - name: Trinidad Sour
    ingredients:
      - angostura bitters
      - orgeat syrup
      - rye
      - lemon juice
    instructions: 30ml bitters, 15ml orgeat syrup, 15ml rye (preferably high proof), 20ml lemon juice. Shake with ice and strian into a cold glass.
  - name: Two-One-Two
    ingredients:
      - tequila reposado
      - aperol
      - grapefruit juice
    instructions: 40ml tequila, 20ml aperol, 40ml grapefruit juice. Shake with ice and strain into a cold glass. Garnish with a grapefruit twist
  - name: Vesper
    ingredients:
      - gin
      - vodka
      - sweet white vermouth
    instructions: 45ml gin, 15ml vodka, 7.5ml vermouth. Shake with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Vowel
    ingredients:
      - scotch
      - sweet vermouth
      - orange juice
      - kummel
      - angostura bitters
    instructions: 30ml scotch, 30ml sweet vermouth, 15ml orange juice, 15ml kummel, dash of bitters. Shake with ice and strain into a cold glass. Garnish with an orange twist.
  - name: Wah-Wah
    ingredients:
      - pisco
      - elderflower liqueur
      - aperol
      - grapefruit juice
      - angostura bitters
    instructions: 40ml pisco, 30ml elderflower liqueur, 20ml aperol, 20ml grapefruit juice, dash of bitters. Shake with ice and strain into a cold glass. Garnish with a grapefruit twist.
  - name: Ward Eight
    ingredients:
      - bourbon
      - lemon juice
      - orange juice
      - grenadine
    instructions: 40ml bourbon, 20ml lemon juice, 20ml orange juice, 2.5ml grenadine. Shake with ice and strain into a cold glass. Garnish with an orange slice.
  - name: White Lady
    ingredients:
      - gin
      - lemon juice
      - orange liqueur
      - egg white
    instructions: 50ml gin, 25ml lemon juice, 20ml orange liqueur, 15ml egg white. Dry shake with no ice until the egg is frothed. Add ice and shake until cool. Strain into a glass.
  - name: La Bicyclette
    ingredients:
      - gin
      - sweet vermouth
      - elderflower liqueur
      - peach bitters
    instructions: 50ml gin, 20ml sweet vermouth, 10ml elderflower liqueur, two dashes of bitters. Stir with ice and strain into a cold glass. Garnish with a lemon twist.
  - name: Bitter Pill
    ingredients:
      - dark rum
      - fernet branca
      - lime juice
      - golden syrup
    instructions: 50ml dark rum, 15ml fernet branca, 15ml lime juice, 10ml syrup. Shake with ice and strain into a cold glass. Garnish with a lime twist.
  - name: Jerry Thomas Gin Punch
    ingredients:
      - gin
      - lemon
      - raw cane sugar
      - cold black tea
      - raspberry syrup
      - raspberry
    instructions: |
      For 10 people.
      1 bottle of gin, 5 lemons, 150g sugar, 500ml tea, 75ml syrup, handful of fresh raspberries.
      Make a block of ice by placing raspberries in a bowl, fill it with water and place in the freezer.
      Remove the skin from the lemons using a peeler, make sure not to include the pith. Place the skins into a punch bowl with teh sugar and pound them together. Let them sit together for half an hour.
      Prepare the tea, removing the tea bags or leaves once it's brewed.
      Make sure to reserve some extra gin, sugar, tea, and lemon from the following steps so that you can adjust the punch to taste.
      Pour the tea over the lemon sugar to dissolve.
      Stir in the juice of the lemons, the syrup, and the gin.
      Balance the punch with the leftovers.
      Add in slices of orange, pineapple, and lemon. Grate over some nutmeg.
      Let cool and add the ice prior to serving with a few mint leaves.
  - name: Ginger Rogers Punch
    ingredients:
      - gin
      - limes
      - mint
      - ginger beer
      - angostura bitters
    instructions: |
      For 6 people.
      300ml gin, 3 limes, 6 sprigs of mint, 500ml ginger beer, many dashes of bitters.
      Bruise the mint at the bottom of the jug.
      Slice the limes into quarters, squeeze their juice in, and then throw in the shells.
      Add the gin and enough ice to rise above the line of liquid.
      Churn the mixture, add in the ginger beer and the bitters.
      Garnish with lime wedge or mint sprig.
";
