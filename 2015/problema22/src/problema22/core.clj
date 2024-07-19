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

(defn apply_shield [state]
  (def newstate {:e {:poison (get (get state :e) :poison),
                     :shield (pop (get (get state :e) :shield)),
                     :recharge (get (get state :e) :recharge)},
                 :php (get state :php),
                 :bhp (get state :bhp),
                 :bdmg (get state :bdmg),
                 :mana (get state :mana)})
  newstate)

(defn shield [state]
  (def newstate {:e {:poison (get (get state :e) :poison),
                     :shield (seq (concat (get (get state :e) :shield) (repeat 6 apply_shield))),
                     :recharge (get (get state :e) :recharge)},
                 :php (+ (get state :php) 2),
                 :bhp (- (get state :bhp) 2),
                 :bdmg (get state :bdmg),
                 :mana (- (get state :mana) 113)})
  newstate) 

(defn tick [state myturn spells]
  (if (and (> (get state :bhp) 0)
           (> (get state :php) 0))
    (do ; boss and me alive
      (if (= myturn true)
        (if (< (get state :mana) 53) 
          -1 ;CANNOT CAST SPELLS
          (if (> (get state :php) 0);myturn
            (do ; alive
              (def actives 1)
              (shield state))
            -1; dead
            ) 
          )
        ))
    (if (<= (get state :bhp) 0); boss or me dead
      (get state :mana) ; boss dead
      -1)  ; me dead
    ))

(defn main
  (do
    (def effects {:poison (),
                  :shield (),
                  :recharge ()}) 
    (def spells [magic-missile drain shield])
   (def game_state {:e effects, :php 50, :bhp 55, :bdmg 8, :mana 500})
   (def res (tick game_state true spells))
   (rest (get (get res :e) :shield))
))
