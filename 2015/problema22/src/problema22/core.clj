(ns problema22.core)


(defn magic-missile [state]
  (def newstate {:e (get state :e),
                 :php (get state :php),
                 :bhp (- (get state :bhp) 4)
                 :bdmg (get state :bdmg),
                 :mana (- (get state :mana) 53)})
  newstate) 


(defn drain [state]
  (def newstate {:e (get state :e),
                 :php (+ (get state :php) 2),
                 :bhp (- (get state :bhp) 2),
                 :bdmg (get state :bdmg),
                 :mana (- (get state :mana) 73)})
  newstate) 

(defn shield [state]
  (def newstate {:e {:poison (get (get state :e) :poison),
                     :shield (seq (concat (get (get state :e) :shield) (repeat 6 1))),
                     :recharge (get (get state :e) :recharge)},
                 :php (get state :php),
                 :bhp (get state :bhp),
                 :bdmg (get state :bdmg),
                 :mana (- (get state :mana) 113)})
  newstate) 


(defn poison [state]
  (def newstate {:e {:poison (seq (concat (get (get state :e) :poison) (repeat 6 1))),
                     :shield (get (get state :e) :shield),
                     :recharge (get (get state :e) :recharge)},
                 :php (get state :php),
                 :bhp (get state :bhp),
                 :bdmg (get state :bdmg),
                 :mana (- (get state :mana) 173)})
  newstate) 


(defn recharge [state]
  (def newstate {:e {:poison (get (get state :e) :poison),
                     :shield (get (get state :e) :shield),
                     :recharge (seq (concat (get (get state :e) :recharge) (repeat 5 1)))},
                 :php (get state :php),
                 :bhp (get state :bhp),
                 :bdmg (get state :bdmg),
                 :mana (- (get state :mana) 229)})
  newstate) 

(defn apply_effects [state]
  (def newstate {:e {:poison (rest (get (get state :e) :poison)),
                     :shield (rest (get (get state :e) :shield)),
                     :recharge (rest (get (get state :e) :recharge))},
                 :php (get state :php),
                 :bhp (- (get state :bhp) (if (= (get (get state :e) :poison) ()) 0 3)),
                 :bdmg (if (= (get (get state :e) :shield) ()) 8 1),
                 :mana (+ (get state :mana) (if (= (get (get state :e) :recharge) ()) 0 101))})
  newstate)

(defn boss_attacks [state]
  (def newstate {:e (get state :e),
                 :php (- (get state :php) (get state :bdmg)),
                 :bhp (get state :bhp),
                 :bdmg (get state :bdmg),
                 :mana (get state :mana)})
  newstate)

(defn permutation [xs]
  (if (= (count xs) 1)
    (list xs)
    (for [x xs y (permutation (disj xs x))] (cons x y))))

(defn available_spells_at_tick [mana]
  (if (> mana 229)
    (permutation (set [recharge poison shield drain magic-missile])) 
    (if (> mana 173)
      (permutation (set [poison shield drain magic-missile]))
      (if (> mana 113)
        (permutation (set [shield drain magic-missile]))
        (if (> mana 73)
          (permutation (set [drain magic-missile]))
          (if (mana > 53)
            [magic-missile]
            []))))))

(defn tick [state myturn spells]
  (do
    (def after_e (apply_effects state))
    (if (and (> (get after_e :bhp) 0)
             (> (get after_e :php) 0))
      (do
        (def available_spells (available_spells_at_tick (get after_e :mana)))
        ) ; BOSS AND ME ALIVE
      (if (<= 0 (get after_e :bhp))
        (do after_e) ; BOSS DEAD
        (do 0) ; ME DEADhh
        ) ; BOSS OR ME DEAD
      )))

(defn main
  (do
    (def effects {:poison (),
                  :shield (),
                  :recharge ()}) 
    (def spells [magic-missile drain shield poison recharge])
   (def game_state {:e effects, :php 50, :bhp 55, :bdmg 8, :mana 500})
   (def res (tick game_state true spells))
    res
))
