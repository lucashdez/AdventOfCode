(ns problema22.core)


(defn magic-missile [state]
  (def newstate {:e (get state :e),
                 :php (get state :php),
                 :bhp (- (get state :bhp) 4)
                 :bdmg (get state :bdmg),
                 :mana (- (get state :mana) 53)})
  newstate) 

(defn log [state]
  (do
    (println state)
    (read-line)))


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
  (def newstate [[(rest (get (get state 0) 0))
                   (rest (get (get state 0) 1))
                   (rest (get (get state 0) 2))]
                 (get state 1)
                 (- (get state 2) (if (= (get (get state 0) 0) ()) 0 3))
                 (if (= (get (get state 0) 1) ()) 8 1)
                 (+ (get state 4) (if (= (get (get state 0) 2) ()) 0 101))
                 ])
  newstate)

(defn boss_attacks [state]
  (def newstate [(get state 0)
                 (- (get state 1) (get state 3))
                 (get state 2)
                 (get state 3)
                 (get state 4)])
  newstate)

(defn permutation [xs]
  (if (= (count xs) 1)
    (list xs)
    (for [x xs y (permutation (disj xs x))] (cons x y))))

(defn available_spells_at_tick [mana]
  (if (>= mana 229)
    (set [recharge poison shield drain magic-missile]))
    (if (>= mana 173)
      (set [poison shield drain magic-missile])
      (if (>= mana 113)
        (set [shield drain magic-missile])
        (if (>= mana 73)
          (set [drain magic-missile])
          (if (>= mana 53)
            [magic-missile]
            [])))))

(defn tick [state myturn spells]
  (do
    (def after_e (apply_effects state))
    (boss_attacks after_e)
    (boss_attacks after_e)
    )
  )

(defn main []
  (do
    (def effects [() () ()]) 
    (def spells [magic-missile drain shield poison recharge])
   (def game_state [effects 10 13 8 250])
   (def res (tick game_state true spells))
   res
   ))
