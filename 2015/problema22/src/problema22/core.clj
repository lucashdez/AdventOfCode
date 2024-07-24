(ns problema22.core)


(defn magic-missile [state]
  [(get state 0)
   (get state 1)
   (- (get state 2) 4)
   (get state 3)
   (- (get state 4) 53)
   (+ (get state 5) 53)])

(defn log [state]
  (do
    (println state)
    (read-line)))


(defn drain [state]
  [(get state 0)
   (+ (get state 1) 2)
   (- (get state 2) 2)
   (get state 3)
   (- (get state 4) 73)
   (+ (get state 5) 73)])

(defn shield [state]
  [[(get (get state 0) 0)
    (seq (concat (get (get state 0) 1) (repeat 6 1)))
    (get (get state 0) 2)]
   (get state 1)
   (get state 2)
   (get state 3)
   (- (get state 4) 113)
   (+ (get state 5) 113)]) 


(defn poison [state]
  [[(seq (concat (get (get state 0) 0) (repeat 6 1)))
    (get (get state 0) 1)
    (get (get state 0) 2)]
   (get state 1)
   (get state 2)
   (get state 3)
   (- (get state 4) 173)
   (+ (get state 5) 173)]) 


(defn recharge [state]
    [[(get (get state 0) 0)
      (get (get state 0) 1)
      (seq (concat (get (get state 0) 2) (repeat 5 1)))]
     (get state 1)
     (get state 2)
     (get state 3)
     (- (get state 4) 229)
     (+ (get state 5) 229)]
) 

(defn apply_effects [state]
  [[(rest (get (get state 0) 0))
    (rest (get (get state 0) 1))
    (rest (get (get state 0) 2))]
   (get state 1)
   (- (get state 2) (if (= (get (get state 0) 0) ()) 0 3))
   (if (= (get (get state 0) 1) ()) 8 1)
   (+ (get state 4) (if (= (get (get state 0) 2) ()) 0 101))
   (get state 5)
   ])

(defn boss_attacks [state]
  [(get state 0)
   (- (get state 1) (get state 3))
   (get state 2)
   (get state 3)
   (get state 4)
   (get state 5)])

(defn permutation [xs]
  (if (= (count xs) 1)
    (list xs)
    (for [x xs y (permutation (disj xs x))] (cons x y))))

(defn available_spells_at_tick [mana]
  (if (>= mana 229)
    (set [recharge poison shield drain magic-missile])
    (if (>= mana 173)
      (set [poison shield drain magic-missile])
      (if (>= mana 113)
        (set [shield drain magic-missile])
        (if (>= mana 73)
          (set [drain magic-missile])
          (if (>= mana 53)
            [magic-missile]
            []))))))

(defn tick [state myturn spells]
  (do
    (let [after_e (apply_effects state)]
      (if (and (> (get after_e 1) 0)
               (> (get after_e 2) 0))
                                        ; ALL ALIVE
        (if (= myturn true)
                                        ; MY TURN
          (for [spell (available_spells_at_tick (get after_e 4))]
            (if (= spell ())
              -1
              (tick (spell after_e) (not myturn) spells)))
                                        ; BOSS TURN
          (tick (boss_attacks after_e) (not myturn) spells))
                                        ; ONE DEAD
        (if (<= (get after_e 2) 0)
                                        ; BOSS DEAD
          (do
            (def end true)
            (get after_e 5))
                                        ; ME DEAD
          -1)))
    )
  )

(defn main []
  (def end false)
  (let [spells [magic-missile drain shield poison recharge]
        ;game_state [[() () ()] 50 55 8 500]
        game_state [[() () ()] 10 13 8 250 0]
        ]
   (flatten (tick game_state true spells))))
