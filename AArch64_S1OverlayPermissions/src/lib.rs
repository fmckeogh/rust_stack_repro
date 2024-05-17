#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use AArch64_S1POR::*;
use HasUnprivileged::*;
use common::*;
pub fn AArch64_S1OverlayPermissions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    walkstate: ProductType96e7acababe246a1,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductTypea231b9ca5c98dc3c {
    #[derive(Default)]
    struct FunctionState {
        ppo: u8,
        ga_12775: ProductTyped8f896a024a4e2cb,
        ga_12760: ProductTyped8f896a024a4e2cb,
        ga_12765: ProductTyped8f896a024a4e2cb,
        ga_12763: ProductTyped8f896a024a4e2cb,
        ga_12771: ProductTyped8f896a024a4e2cb,
        permissions: ProductTypebf05c51f33174538,
        ga_12774: ProductTyped8f896a024a4e2cb,
        ga_12770: ProductTyped8f896a024a4e2cb,
        ga_12762: ProductTyped8f896a024a4e2cb,
        ux: bool,
        pw: bool,
        ga_12759: ProductTyped8f896a024a4e2cb,
        ga_12758: ProductTyped8f896a024a4e2cb,
        px: bool,
        por: ProductType5c790c8ef59cc8b2,
        bit_index: i64,
        pr: bool,
        ga_12769: ProductTyped8f896a024a4e2cb,
        ga_12764: ProductTyped8f896a024a4e2cb,
        ga_12766: ProductTyped8f896a024a4e2cb,
        ga_12768: ProductTyped8f896a024a4e2cb,
        ur: bool,
        ga_12773: ProductTyped8f896a024a4e2cb,
        ga_12772: ProductTyped8f896a024a4e2cb,
        ga_12761: ProductTyped8f896a024a4e2cb,
        ga_12780: ProductTyped8f896a024a4e2cb,
        upo: u8,
        s1overlay_perms: ProductTypea231b9ca5c98dc3c,
        ga_12776: ProductTyped8f896a024a4e2cb,
        uw: bool,
        regime: u32,
        walkstate: ProductType96e7acababe246a1,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        regime,
        walkstate,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_0_0: read-var walkstate.9:struct
        let s_0_0: ProductTypebf05c51f33174538 = fn_state.walkstate._9;
        // D s_0_1: write-var permissions <= s_0_0
        fn_state.permissions = s_0_0;
        // D s_0_2: read-var regime:u32
        let s_0_2: u32 = fn_state.regime;
        // D s_0_3: call AArch64_S1POR(s_0_2)
        let s_0_3: ProductType5c790c8ef59cc8b2 = AArch64_S1POR(state, tracer, s_0_2);
        // D s_0_4: write-var por <= s_0_3
        fn_state.por = s_0_3;
        // D s_0_5: read-var permissions.3:struct
        let s_0_5: u8 = fn_state.permissions._3;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 4u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // C s_0_9: const #4s : i
        let s_0_9: i128 = 4;
        // D s_0_10: cast zx s_0_8 -> i
        let s_0_10: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_11: mul s_0_9 s_0_10
        let s_0_11: i128 = ((s_0_9) * (s_0_10));
        // D s_0_12: cast reint s_0_11 -> i64
        let s_0_12: i64 = (s_0_11 as i64);
        // D s_0_13: write-var bit_index <= s_0_12
        fn_state.bit_index = s_0_12;
        // D s_0_14: read-var por.0:struct
        let s_0_14: u64 = fn_state.por._0;
        // C s_0_15: const #4s : i
        let s_0_15: i128 = 4;
        // D s_0_16: cast zx s_0_14 -> bv
        let s_0_16: Bits = Bits::new(s_0_14 as u128, 64u16);
        // D s_0_17: read-var bit_index:i64
        let s_0_17: i64 = fn_state.bit_index;
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_19: bit-extract s_0_16 s_0_18 s_0_15
        let s_0_19: Bits = (Bits::new(
            ((s_0_16) >> (s_0_18)).value(),
            u16::try_from(s_0_15).unwrap(),
        ));
        // D s_0_20: cast reint s_0_19 -> u8
        let s_0_20: u8 = (s_0_19.value() as u8);
        // D s_0_21: write-var ppo <= s_0_20
        fn_state.ppo = s_0_20;
        // D s_0_22: read-var ppo:u8
        let s_0_22: u8 = fn_state.ppo;
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 4u16);
        // C s_0_24: const #0u : u8
        let s_0_24: u8 = 0;
        // C s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 4u16);
        // D s_0_26: cmp-eq s_0_23 s_0_25
        let s_0_26: bool = ((s_0_23) == (s_0_25));
        // D s_0_27: not s_0_26
        let s_0_27: bool = !s_0_26;
        // N s_0_28: branch s_0_27 b26 b1
        if s_0_27 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // C s_1_1: const #0u : u8
        let s_1_1: bool = false;
        // C s_1_2: const #0u : u8
        let s_1_2: bool = false;
        // D s_1_3: create-product struct = ["s_1_0", "s_1_1", "s_1_2"]
        let s_1_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_1_0,
            _1: s_1_1,
            _2: s_1_2,
        };
        // D s_1_4: write-var ga#12758 <= s_1_3
        fn_state.ga_12758 = s_1_3;
        // D s_1_5: read-var ga#12758.0:struct
        let s_1_5: bool = fn_state.ga_12758._0;
        // D s_1_6: read-var ga#12758.1:struct
        let s_1_6: bool = fn_state.ga_12758._1;
        // D s_1_7: read-var ga#12758.2:struct
        let s_1_7: bool = fn_state.ga_12758._2;
        // D s_1_8: write-var pr <= s_1_5
        fn_state.pr = s_1_5;
        // D s_1_9: write-var pw <= s_1_6
        fn_state.pw = s_1_6;
        // D s_1_10: write-var px <= s_1_7
        fn_state.px = s_1_7;
        // N s_1_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_2_0: read-var regime:u32
        let s_2_0: u32 = fn_state.regime;
        // D s_2_1: call HasUnprivileged(s_2_0)
        let s_2_1: bool = HasUnprivileged(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b8 b3
        if s_2_1 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_4_0: read-var accdesc.8:struct
        let s_4_0: u8 = fn_state.accdesc._8;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #448u : u32
        let s_4_2: u32 = 448;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b7 b5
        if s_4_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_5_0: read-var pr:u8
        let s_5_0: bool = fn_state.pr;
        // D s_5_1: read-var pw:u8
        let s_5_1: bool = fn_state.pw;
        // D s_5_2: read-var px:u8
        let s_5_2: bool = fn_state.px;
        // D s_5_3: create-product struct = ["s_5_0", "s_5_1", "s_5_2"]
        let s_5_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_5_0,
            _1: s_5_1,
            _2: s_5_2,
        };
        // D s_5_4: write-var ga#12780 <= s_5_3
        fn_state.ga_12780 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_6_0: read-var ga#12780.0:struct
        let s_6_0: bool = fn_state.ga_12780._0;
        // D s_6_1: read-var ga#12780.1:struct
        let s_6_1: bool = fn_state.ga_12780._1;
        // D s_6_2: read-var ga#12780.2:struct
        let s_6_2: bool = fn_state.ga_12780._2;
        // D s_6_3: write-var s1overlay_perms.1 <= s_6_0
        fn_state.s1overlay_perms._1 = s_6_0;
        // D s_6_4: write-var s1overlay_perms.3 <= s_6_1
        fn_state.s1overlay_perms._3 = s_6_1;
        // D s_6_5: write-var s1overlay_perms.4 <= s_6_2
        fn_state.s1overlay_perms._4 = s_6_2;
        // D s_6_6: read-var s1overlay_perms:struct
        let s_6_6: ProductTypea231b9ca5c98dc3c = fn_state.s1overlay_perms;
        // N s_6_7: return s_6_6
        return s_6_6;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_7_0: read-var ur:u8
        let s_7_0: bool = fn_state.ur;
        // D s_7_1: read-var uw:u8
        let s_7_1: bool = fn_state.uw;
        // D s_7_2: read-var ux:u8
        let s_7_2: bool = fn_state.ux;
        // D s_7_3: create-product struct = ["s_7_0", "s_7_1", "s_7_2"]
        let s_7_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_7_0,
            _1: s_7_1,
            _2: s_7_2,
        };
        // D s_7_4: write-var ga#12780 <= s_7_3
        fn_state.ga_12780 = s_7_3;
        // N s_7_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_8_0: const #102688u : u32
        let s_8_0: u32 = 102688;
        // D s_8_1: read-reg s_8_0:u64
        let s_8_1: u64 = {
            let value = state.read_register::<u64>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #4s : i
        let s_8_2: i128 = 4;
        // D s_8_3: cast zx s_8_1 -> bv
        let s_8_3: Bits = Bits::new(s_8_1 as u128, 64u16);
        // D s_8_4: read-var bit_index:i64
        let s_8_4: i64 = fn_state.bit_index;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: bit-extract s_8_3 s_8_5 s_8_2
        let s_8_6: Bits = (Bits::new(
            ((s_8_3) >> (s_8_5)).value(),
            u16::try_from(s_8_2).unwrap(),
        ));
        // D s_8_7: cast reint s_8_6 -> u8
        let s_8_7: u8 = (s_8_6.value() as u8);
        // D s_8_8: write-var upo <= s_8_7
        fn_state.upo = s_8_7;
        // D s_8_9: read-var upo:u8
        let s_8_9: u8 = fn_state.upo;
        // D s_8_10: cast zx s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 4u16);
        // C s_8_11: const #0u : u8
        let s_8_11: u8 = 0;
        // C s_8_12: cast zx s_8_11 -> bv
        let s_8_12: Bits = Bits::new(s_8_11 as u128, 4u16);
        // D s_8_13: cmp-eq s_8_10 s_8_12
        let s_8_13: bool = ((s_8_10) == (s_8_12));
        // D s_8_14: not s_8_13
        let s_8_14: bool = !s_8_13;
        // N s_8_15: branch s_8_14 b11 b9
        if s_8_14 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // C s_9_1: const #0u : u8
        let s_9_1: bool = false;
        // C s_9_2: const #0u : u8
        let s_9_2: bool = false;
        // D s_9_3: create-product struct = ["s_9_0", "s_9_1", "s_9_2"]
        let s_9_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_9_0,
            _1: s_9_1,
            _2: s_9_2,
        };
        // D s_9_4: write-var ga#12768 <= s_9_3
        fn_state.ga_12768 = s_9_3;
        // D s_9_5: read-var ga#12768.0:struct
        let s_9_5: bool = fn_state.ga_12768._0;
        // D s_9_6: read-var ga#12768.1:struct
        let s_9_6: bool = fn_state.ga_12768._1;
        // D s_9_7: read-var ga#12768.2:struct
        let s_9_7: bool = fn_state.ga_12768._2;
        // D s_9_8: write-var ur <= s_9_5
        fn_state.ur = s_9_5;
        // D s_9_9: write-var uw <= s_9_6
        fn_state.uw = s_9_6;
        // D s_9_10: write-var ux <= s_9_7
        fn_state.ux = s_9_7;
        // N s_9_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_10_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_11_0: read-var upo:u8
        let s_11_0: u8 = fn_state.upo;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 4u16);
        // C s_11_2: const #1u : u8
        let s_11_2: u8 = 1;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 4u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // C s_12_1: const #0u : u8
        let s_12_1: bool = false;
        // C s_12_2: const #0u : u8
        let s_12_2: bool = false;
        // D s_12_3: create-product struct = ["s_12_0", "s_12_1", "s_12_2"]
        let s_12_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_12_0,
            _1: s_12_1,
            _2: s_12_2,
        };
        // D s_12_4: write-var ga#12769 <= s_12_3
        fn_state.ga_12769 = s_12_3;
        // D s_12_5: read-var ga#12769.0:struct
        let s_12_5: bool = fn_state.ga_12769._0;
        // D s_12_6: read-var ga#12769.1:struct
        let s_12_6: bool = fn_state.ga_12769._1;
        // D s_12_7: read-var ga#12769.2:struct
        let s_12_7: bool = fn_state.ga_12769._2;
        // D s_12_8: write-var ur <= s_12_5
        fn_state.ur = s_12_5;
        // D s_12_9: write-var uw <= s_12_6
        fn_state.uw = s_12_6;
        // D s_12_10: write-var ux <= s_12_7
        fn_state.ux = s_12_7;
        // N s_12_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_13_0: read-var upo:u8
        let s_13_0: u8 = fn_state.upo;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 4u16);
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 4u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // C s_14_1: const #0u : u8
        let s_14_1: bool = false;
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // D s_14_3: create-product struct = ["s_14_0", "s_14_1", "s_14_2"]
        let s_14_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_14_0,
            _1: s_14_1,
            _2: s_14_2,
        };
        // D s_14_4: write-var ga#12770 <= s_14_3
        fn_state.ga_12770 = s_14_3;
        // D s_14_5: read-var ga#12770.0:struct
        let s_14_5: bool = fn_state.ga_12770._0;
        // D s_14_6: read-var ga#12770.1:struct
        let s_14_6: bool = fn_state.ga_12770._1;
        // D s_14_7: read-var ga#12770.2:struct
        let s_14_7: bool = fn_state.ga_12770._2;
        // D s_14_8: write-var ur <= s_14_5
        fn_state.ur = s_14_5;
        // D s_14_9: write-var uw <= s_14_6
        fn_state.uw = s_14_6;
        // D s_14_10: write-var ux <= s_14_7
        fn_state.ux = s_14_7;
        // N s_14_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_15_0: read-var upo:u8
        let s_15_0: u8 = fn_state.upo;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 4u16);
        // C s_15_2: const #3u : u8
        let s_15_2: u8 = 3;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 4u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b17 b16
        if s_15_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // C s_16_1: const #0u : u8
        let s_16_1: bool = false;
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // D s_16_3: create-product struct = ["s_16_0", "s_16_1", "s_16_2"]
        let s_16_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_16_0,
            _1: s_16_1,
            _2: s_16_2,
        };
        // D s_16_4: write-var ga#12771 <= s_16_3
        fn_state.ga_12771 = s_16_3;
        // D s_16_5: read-var ga#12771.0:struct
        let s_16_5: bool = fn_state.ga_12771._0;
        // D s_16_6: read-var ga#12771.1:struct
        let s_16_6: bool = fn_state.ga_12771._1;
        // D s_16_7: read-var ga#12771.2:struct
        let s_16_7: bool = fn_state.ga_12771._2;
        // D s_16_8: write-var ur <= s_16_5
        fn_state.ur = s_16_5;
        // D s_16_9: write-var uw <= s_16_6
        fn_state.uw = s_16_6;
        // D s_16_10: write-var ux <= s_16_7
        fn_state.ux = s_16_7;
        // N s_16_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_17_0: read-var upo:u8
        let s_17_0: u8 = fn_state.upo;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 4u16);
        // C s_17_2: const #4u : u8
        let s_17_2: u8 = 4;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 4u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b19 b18
        if s_17_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // C s_18_1: const #1u : u8
        let s_18_1: bool = true;
        // C s_18_2: const #0u : u8
        let s_18_2: bool = false;
        // D s_18_3: create-product struct = ["s_18_0", "s_18_1", "s_18_2"]
        let s_18_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_18_0,
            _1: s_18_1,
            _2: s_18_2,
        };
        // D s_18_4: write-var ga#12772 <= s_18_3
        fn_state.ga_12772 = s_18_3;
        // D s_18_5: read-var ga#12772.0:struct
        let s_18_5: bool = fn_state.ga_12772._0;
        // D s_18_6: read-var ga#12772.1:struct
        let s_18_6: bool = fn_state.ga_12772._1;
        // D s_18_7: read-var ga#12772.2:struct
        let s_18_7: bool = fn_state.ga_12772._2;
        // D s_18_8: write-var ur <= s_18_5
        fn_state.ur = s_18_5;
        // D s_18_9: write-var uw <= s_18_6
        fn_state.uw = s_18_6;
        // D s_18_10: write-var ux <= s_18_7
        fn_state.ux = s_18_7;
        // N s_18_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_19_0: read-var upo:u8
        let s_19_0: u8 = fn_state.upo;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 4u16);
        // C s_19_2: const #5u : u8
        let s_19_2: u8 = 5;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 4u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: not s_19_4
        let s_19_5: bool = !s_19_4;
        // N s_19_6: branch s_19_5 b21 b20
        if s_19_5 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // C s_20_1: const #1u : u8
        let s_20_1: bool = true;
        // C s_20_2: const #0u : u8
        let s_20_2: bool = false;
        // D s_20_3: create-product struct = ["s_20_0", "s_20_1", "s_20_2"]
        let s_20_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_20_0,
            _1: s_20_1,
            _2: s_20_2,
        };
        // D s_20_4: write-var ga#12773 <= s_20_3
        fn_state.ga_12773 = s_20_3;
        // D s_20_5: read-var ga#12773.0:struct
        let s_20_5: bool = fn_state.ga_12773._0;
        // D s_20_6: read-var ga#12773.1:struct
        let s_20_6: bool = fn_state.ga_12773._1;
        // D s_20_7: read-var ga#12773.2:struct
        let s_20_7: bool = fn_state.ga_12773._2;
        // D s_20_8: write-var ur <= s_20_5
        fn_state.ur = s_20_5;
        // D s_20_9: write-var uw <= s_20_6
        fn_state.uw = s_20_6;
        // D s_20_10: write-var ux <= s_20_7
        fn_state.ux = s_20_7;
        // N s_20_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_21_0: read-var upo:u8
        let s_21_0: u8 = fn_state.upo;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 4u16);
        // C s_21_2: const #6u : u8
        let s_21_2: u8 = 6;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 4u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b23 b22
        if s_21_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // C s_22_1: const #1u : u8
        let s_22_1: bool = true;
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // D s_22_3: create-product struct = ["s_22_0", "s_22_1", "s_22_2"]
        let s_22_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_22_0,
            _1: s_22_1,
            _2: s_22_2,
        };
        // D s_22_4: write-var ga#12774 <= s_22_3
        fn_state.ga_12774 = s_22_3;
        // D s_22_5: read-var ga#12774.0:struct
        let s_22_5: bool = fn_state.ga_12774._0;
        // D s_22_6: read-var ga#12774.1:struct
        let s_22_6: bool = fn_state.ga_12774._1;
        // D s_22_7: read-var ga#12774.2:struct
        let s_22_7: bool = fn_state.ga_12774._2;
        // D s_22_8: write-var ur <= s_22_5
        fn_state.ur = s_22_5;
        // D s_22_9: write-var uw <= s_22_6
        fn_state.uw = s_22_6;
        // D s_22_10: write-var ux <= s_22_7
        fn_state.ux = s_22_7;
        // N s_22_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_23_0: read-var upo:u8
        let s_23_0: u8 = fn_state.upo;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 4u16);
        // C s_23_2: const #7u : u8
        let s_23_2: u8 = 7;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 4u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: not s_23_4
        let s_23_5: bool = !s_23_4;
        // N s_23_6: branch s_23_5 b25 b24
        if s_23_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // C s_24_1: const #1u : u8
        let s_24_1: bool = true;
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // D s_24_3: create-product struct = ["s_24_0", "s_24_1", "s_24_2"]
        let s_24_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_24_0,
            _1: s_24_1,
            _2: s_24_2,
        };
        // D s_24_4: write-var ga#12775 <= s_24_3
        fn_state.ga_12775 = s_24_3;
        // D s_24_5: read-var ga#12775.0:struct
        let s_24_5: bool = fn_state.ga_12775._0;
        // D s_24_6: read-var ga#12775.1:struct
        let s_24_6: bool = fn_state.ga_12775._1;
        // D s_24_7: read-var ga#12775.2:struct
        let s_24_7: bool = fn_state.ga_12775._2;
        // D s_24_8: write-var ur <= s_24_5
        fn_state.ur = s_24_5;
        // D s_24_9: write-var uw <= s_24_6
        fn_state.uw = s_24_6;
        // D s_24_10: write-var ux <= s_24_7
        fn_state.ux = s_24_7;
        // N s_24_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // C s_25_1: const #0u : u8
        let s_25_1: bool = false;
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // D s_25_3: create-product struct = ["s_25_0", "s_25_1", "s_25_2"]
        let s_25_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_25_0,
            _1: s_25_1,
            _2: s_25_2,
        };
        // D s_25_4: write-var ga#12776 <= s_25_3
        fn_state.ga_12776 = s_25_3;
        // D s_25_5: read-var ga#12776.0:struct
        let s_25_5: bool = fn_state.ga_12776._0;
        // D s_25_6: read-var ga#12776.1:struct
        let s_25_6: bool = fn_state.ga_12776._1;
        // D s_25_7: read-var ga#12776.2:struct
        let s_25_7: bool = fn_state.ga_12776._2;
        // D s_25_8: write-var ur <= s_25_5
        fn_state.ur = s_25_5;
        // D s_25_9: write-var uw <= s_25_6
        fn_state.uw = s_25_6;
        // D s_25_10: write-var ux <= s_25_7
        fn_state.ux = s_25_7;
        // N s_25_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_26_0: read-var ppo:u8
        let s_26_0: u8 = fn_state.ppo;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 4u16);
        // C s_26_2: const #1u : u8
        let s_26_2: u8 = 1;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 4u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: not s_26_4
        let s_26_5: bool = !s_26_4;
        // N s_26_6: branch s_26_5 b28 b27
        if s_26_5 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // C s_27_1: const #0u : u8
        let s_27_1: bool = false;
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // D s_27_3: create-product struct = ["s_27_0", "s_27_1", "s_27_2"]
        let s_27_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_27_0,
            _1: s_27_1,
            _2: s_27_2,
        };
        // D s_27_4: write-var ga#12759 <= s_27_3
        fn_state.ga_12759 = s_27_3;
        // D s_27_5: read-var ga#12759.0:struct
        let s_27_5: bool = fn_state.ga_12759._0;
        // D s_27_6: read-var ga#12759.1:struct
        let s_27_6: bool = fn_state.ga_12759._1;
        // D s_27_7: read-var ga#12759.2:struct
        let s_27_7: bool = fn_state.ga_12759._2;
        // D s_27_8: write-var pr <= s_27_5
        fn_state.pr = s_27_5;
        // D s_27_9: write-var pw <= s_27_6
        fn_state.pw = s_27_6;
        // D s_27_10: write-var px <= s_27_7
        fn_state.px = s_27_7;
        // N s_27_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_28_0: read-var ppo:u8
        let s_28_0: u8 = fn_state.ppo;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 4u16);
        // C s_28_2: const #2u : u8
        let s_28_2: u8 = 2;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 4u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: not s_28_4
        let s_28_5: bool = !s_28_4;
        // N s_28_6: branch s_28_5 b30 b29
        if s_28_5 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // C s_29_1: const #0u : u8
        let s_29_1: bool = false;
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // D s_29_3: create-product struct = ["s_29_0", "s_29_1", "s_29_2"]
        let s_29_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_29_0,
            _1: s_29_1,
            _2: s_29_2,
        };
        // D s_29_4: write-var ga#12760 <= s_29_3
        fn_state.ga_12760 = s_29_3;
        // D s_29_5: read-var ga#12760.0:struct
        let s_29_5: bool = fn_state.ga_12760._0;
        // D s_29_6: read-var ga#12760.1:struct
        let s_29_6: bool = fn_state.ga_12760._1;
        // D s_29_7: read-var ga#12760.2:struct
        let s_29_7: bool = fn_state.ga_12760._2;
        // D s_29_8: write-var pr <= s_29_5
        fn_state.pr = s_29_5;
        // D s_29_9: write-var pw <= s_29_6
        fn_state.pw = s_29_6;
        // D s_29_10: write-var px <= s_29_7
        fn_state.px = s_29_7;
        // N s_29_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_30_0: read-var ppo:u8
        let s_30_0: u8 = fn_state.ppo;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 4u16);
        // C s_30_2: const #3u : u8
        let s_30_2: u8 = 3;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 4u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: not s_30_4
        let s_30_5: bool = !s_30_4;
        // N s_30_6: branch s_30_5 b32 b31
        if s_30_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // C s_31_1: const #0u : u8
        let s_31_1: bool = false;
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // D s_31_3: create-product struct = ["s_31_0", "s_31_1", "s_31_2"]
        let s_31_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_31_0,
            _1: s_31_1,
            _2: s_31_2,
        };
        // D s_31_4: write-var ga#12761 <= s_31_3
        fn_state.ga_12761 = s_31_3;
        // D s_31_5: read-var ga#12761.0:struct
        let s_31_5: bool = fn_state.ga_12761._0;
        // D s_31_6: read-var ga#12761.1:struct
        let s_31_6: bool = fn_state.ga_12761._1;
        // D s_31_7: read-var ga#12761.2:struct
        let s_31_7: bool = fn_state.ga_12761._2;
        // D s_31_8: write-var pr <= s_31_5
        fn_state.pr = s_31_5;
        // D s_31_9: write-var pw <= s_31_6
        fn_state.pw = s_31_6;
        // D s_31_10: write-var px <= s_31_7
        fn_state.px = s_31_7;
        // N s_31_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_32_0: read-var ppo:u8
        let s_32_0: u8 = fn_state.ppo;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 4u16);
        // C s_32_2: const #4u : u8
        let s_32_2: u8 = 4;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 4u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: not s_32_4
        let s_32_5: bool = !s_32_4;
        // N s_32_6: branch s_32_5 b34 b33
        if s_32_5 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // C s_33_1: const #1u : u8
        let s_33_1: bool = true;
        // C s_33_2: const #0u : u8
        let s_33_2: bool = false;
        // D s_33_3: create-product struct = ["s_33_0", "s_33_1", "s_33_2"]
        let s_33_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_33_0,
            _1: s_33_1,
            _2: s_33_2,
        };
        // D s_33_4: write-var ga#12762 <= s_33_3
        fn_state.ga_12762 = s_33_3;
        // D s_33_5: read-var ga#12762.0:struct
        let s_33_5: bool = fn_state.ga_12762._0;
        // D s_33_6: read-var ga#12762.1:struct
        let s_33_6: bool = fn_state.ga_12762._1;
        // D s_33_7: read-var ga#12762.2:struct
        let s_33_7: bool = fn_state.ga_12762._2;
        // D s_33_8: write-var pr <= s_33_5
        fn_state.pr = s_33_5;
        // D s_33_9: write-var pw <= s_33_6
        fn_state.pw = s_33_6;
        // D s_33_10: write-var px <= s_33_7
        fn_state.px = s_33_7;
        // N s_33_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_34_0: read-var ppo:u8
        let s_34_0: u8 = fn_state.ppo;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 4u16);
        // C s_34_2: const #5u : u8
        let s_34_2: u8 = 5;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 4u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: not s_34_4
        let s_34_5: bool = !s_34_4;
        // N s_34_6: branch s_34_5 b36 b35
        if s_34_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // C s_35_1: const #1u : u8
        let s_35_1: bool = true;
        // C s_35_2: const #0u : u8
        let s_35_2: bool = false;
        // D s_35_3: create-product struct = ["s_35_0", "s_35_1", "s_35_2"]
        let s_35_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_35_0,
            _1: s_35_1,
            _2: s_35_2,
        };
        // D s_35_4: write-var ga#12763 <= s_35_3
        fn_state.ga_12763 = s_35_3;
        // D s_35_5: read-var ga#12763.0:struct
        let s_35_5: bool = fn_state.ga_12763._0;
        // D s_35_6: read-var ga#12763.1:struct
        let s_35_6: bool = fn_state.ga_12763._1;
        // D s_35_7: read-var ga#12763.2:struct
        let s_35_7: bool = fn_state.ga_12763._2;
        // D s_35_8: write-var pr <= s_35_5
        fn_state.pr = s_35_5;
        // D s_35_9: write-var pw <= s_35_6
        fn_state.pw = s_35_6;
        // D s_35_10: write-var px <= s_35_7
        fn_state.px = s_35_7;
        // N s_35_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_36_0: read-var ppo:u8
        let s_36_0: u8 = fn_state.ppo;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 4u16);
        // C s_36_2: const #6u : u8
        let s_36_2: u8 = 6;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 4u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: not s_36_4
        let s_36_5: bool = !s_36_4;
        // N s_36_6: branch s_36_5 b38 b37
        if s_36_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // C s_37_1: const #1u : u8
        let s_37_1: bool = true;
        // C s_37_2: const #1u : u8
        let s_37_2: bool = true;
        // D s_37_3: create-product struct = ["s_37_0", "s_37_1", "s_37_2"]
        let s_37_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_37_0,
            _1: s_37_1,
            _2: s_37_2,
        };
        // D s_37_4: write-var ga#12764 <= s_37_3
        fn_state.ga_12764 = s_37_3;
        // D s_37_5: read-var ga#12764.0:struct
        let s_37_5: bool = fn_state.ga_12764._0;
        // D s_37_6: read-var ga#12764.1:struct
        let s_37_6: bool = fn_state.ga_12764._1;
        // D s_37_7: read-var ga#12764.2:struct
        let s_37_7: bool = fn_state.ga_12764._2;
        // D s_37_8: write-var pr <= s_37_5
        fn_state.pr = s_37_5;
        // D s_37_9: write-var pw <= s_37_6
        fn_state.pw = s_37_6;
        // D s_37_10: write-var px <= s_37_7
        fn_state.px = s_37_7;
        // N s_37_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_38_0: read-var ppo:u8
        let s_38_0: u8 = fn_state.ppo;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 4u16);
        // C s_38_2: const #7u : u8
        let s_38_2: u8 = 7;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 4u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: not s_38_4
        let s_38_5: bool = !s_38_4;
        // N s_38_6: branch s_38_5 b40 b39
        if s_38_5 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // C s_39_1: const #1u : u8
        let s_39_1: bool = true;
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // D s_39_3: create-product struct = ["s_39_0", "s_39_1", "s_39_2"]
        let s_39_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_39_0,
            _1: s_39_1,
            _2: s_39_2,
        };
        // D s_39_4: write-var ga#12765 <= s_39_3
        fn_state.ga_12765 = s_39_3;
        // D s_39_5: read-var ga#12765.0:struct
        let s_39_5: bool = fn_state.ga_12765._0;
        // D s_39_6: read-var ga#12765.1:struct
        let s_39_6: bool = fn_state.ga_12765._1;
        // D s_39_7: read-var ga#12765.2:struct
        let s_39_7: bool = fn_state.ga_12765._2;
        // D s_39_8: write-var pr <= s_39_5
        fn_state.pr = s_39_5;
        // D s_39_9: write-var pw <= s_39_6
        fn_state.pw = s_39_6;
        // D s_39_10: write-var px <= s_39_7
        fn_state.px = s_39_7;
        // N s_39_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // C s_40_1: const #0u : u8
        let s_40_1: bool = false;
        // C s_40_2: const #0u : u8
        let s_40_2: bool = false;
        // D s_40_3: create-product struct = ["s_40_0", "s_40_1", "s_40_2"]
        let s_40_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_40_0,
            _1: s_40_1,
            _2: s_40_2,
        };
        // D s_40_4: write-var ga#12766 <= s_40_3
        fn_state.ga_12766 = s_40_3;
        // D s_40_5: read-var ga#12766.0:struct
        let s_40_5: bool = fn_state.ga_12766._0;
        // D s_40_6: read-var ga#12766.1:struct
        let s_40_6: bool = fn_state.ga_12766._1;
        // D s_40_7: read-var ga#12766.2:struct
        let s_40_7: bool = fn_state.ga_12766._2;
        // D s_40_8: write-var pr <= s_40_5
        fn_state.pr = s_40_5;
        // D s_40_9: write-var pw <= s_40_6
        fn_state.pw = s_40_6;
        // D s_40_10: write-var px <= s_40_7
        fn_state.px = s_40_7;
        // N s_40_11: jump b2
        return block_2(state, tracer, fn_state);
    }
}
