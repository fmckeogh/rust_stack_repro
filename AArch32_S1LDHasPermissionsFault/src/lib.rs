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
use ConstrainUnpredictable::*;
use HavePANExt::*;
use HasUnprivileged::*;
use common::*;
pub fn AArch32_S1LDHasPermissionsFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    walkparams: ProductTypeef284266e139aee2,
    perms: ProductTypebf05c51f33174538,
    memtype: u32,
    paspace: u32,
    accdesc: ProductType9878976b5bcce9c9,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_21414: u8,
        r: bool,
        ga_21418: ProductTypede60d0d1f6e7c94c,
        ga_21409: u8,
        ga_21410: ProductTypede60d0d1f6e7c94c,
        ga_21457: ProductType8b847afc727d5818,
        ga_21458: ProductType8b847afc727d5818,
        ga_21427: ProductTyped8f896a024a4e2cb,
        ga_21412: ProductTypede60d0d1f6e7c94c,
        pxshadow_513: bool,
        return_value: bool,
        ga_21416: ProductTypede60d0d1f6e7c94c,
        ga_21413: ProductTypede60d0d1f6e7c94c,
        w: bool,
        ga_21453: ProductType8b847afc727d5818,
        x: bool,
        ga_21417: ProductTypede60d0d1f6e7c94c,
        pw: bool,
        gs_27810: bool,
        pr: bool,
        gs_27817: bool,
        ga_21452: ProductType8b847afc727d5818,
        ur: bool,
        ga_21411: ProductTypede60d0d1f6e7c94c,
        gs_27823: bool,
        ga_21415: ProductTypede60d0d1f6e7c94c,
        uxshadow_512: bool,
        gs_27822: bool,
        uw: bool,
        regime: u32,
        walkparams: ProductTypeef284266e139aee2,
        perms: ProductTypebf05c51f33174538,
        memtype: u32,
        paspace: u32,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        regime,
        walkparams,
        perms,
        memtype,
        paspace,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var regime:u32
        let s_0_0: u32 = fn_state.regime;
        // D s_0_1: call HasUnprivileged(s_0_0)
        let s_0_1: bool = HasUnprivileged(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b24 b1
        if s_0_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var perms.0:struct
        let s_1_0: u8 = fn_state.perms._0;
        // C s_1_1: const #2s : i
        let s_1_1: i128 = 2;
        // D s_1_2: cast zx s_1_0 -> bv
        let s_1_2: Bits = Bits::new(s_1_0 as u128, 3u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_1 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_1)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // C s_1_19: const #0u : u8
        let s_1_19: bool = false;
        // C s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // D s_1_21: cmp-eq s_1_18 s_1_20
        let s_1_21: bool = ((s_1_18) == (s_1_20));
        // D s_1_22: not s_1_21
        let s_1_22: bool = !s_1_21;
        // N s_1_23: branch s_1_22 b23 b2
        if s_1_22 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // C s_2_1: const #1u : u8
        let s_2_1: bool = true;
        // D s_2_2: create-product struct = ["s_2_0", "s_2_1"]
        let s_2_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_2_0,
            _1: s_2_1,
        };
        // D s_2_3: write-var ga#21452 <= s_2_2
        fn_state.ga_21452 = s_2_2;
        // D s_2_4: read-var ga#21452.0:struct
        let s_2_4: bool = fn_state.ga_21452._0;
        // D s_2_5: read-var ga#21452.1:struct
        let s_2_5: bool = fn_state.ga_21452._1;
        // D s_2_6: write-var r <= s_2_4
        fn_state.r = s_2_4;
        // D s_2_7: write-var w <= s_2_5
        fn_state.w = s_2_5;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var perms.1:struct
        let s_3_0: u8 = fn_state.perms._1;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: cast zx s_3_0 -> bv
        let s_3_2: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_1 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_1)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // C s_3_19: const #0u : u8
        let s_3_19: bool = false;
        // C s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // D s_3_21: cmp-eq s_3_18 s_3_20
        let s_3_21: bool = ((s_3_18) == (s_3_20));
        // D s_3_22: not s_3_21
        let s_3_22: bool = !s_3_21;
        // N s_3_23: branch s_3_22 b22 b4
        if s_3_22 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var r:u8
        let s_4_0: bool = fn_state.r;
        // D s_4_1: read-var w:u8
        let s_4_1: bool = fn_state.w;
        // D s_4_2: create-product struct = ["s_4_0", "s_4_1"]
        let s_4_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_4_0,
            _1: s_4_1,
        };
        // D s_4_3: write-var ga#21457 <= s_4_2
        fn_state.ga_21457 = s_4_2;
        // D s_4_4: read-var ga#21457.0:struct
        let s_4_4: bool = fn_state.ga_21457._0;
        // D s_4_5: read-var ga#21457.1:struct
        let s_4_5: bool = fn_state.ga_21457._1;
        // D s_4_6: write-var r <= s_4_4
        fn_state.r = s_4_4;
        // D s_4_7: write-var w <= s_4_5
        fn_state.w = s_4_5;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var perms.17:struct
        let s_5_0: bool = fn_state.perms._17;
        // D s_5_1: read-var perms.18:struct
        let s_5_1: bool = fn_state.perms._18;
        // D s_5_2: cast zx s_5_0 -> bv
        let s_5_2: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 1u16);
        // D s_5_4: or s_5_2 s_5_3
        let s_5_4: Bits = ((s_5_2) | (s_5_3));
        // D s_5_5: cast reint s_5_4 -> u8
        let s_5_5: bool = ((s_5_4.value()) != 0);
        // D s_5_6: read-var walkparams.39:struct
        let s_5_6: bool = fn_state.walkparams._39;
        // D s_5_7: read-var w:u8
        let s_5_7: bool = fn_state.w;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 1u16);
        // D s_5_9: cast zx s_5_6 -> bv
        let s_5_9: Bits = Bits::new(s_5_6 as u128, 1u16);
        // D s_5_10: and s_5_8 s_5_9
        let s_5_10: Bits = ((s_5_8) & (s_5_9));
        // D s_5_11: cast reint s_5_10 -> u8
        let s_5_11: bool = ((s_5_10.value()) != 0);
        // D s_5_12: cast zx s_5_5 -> bv
        let s_5_12: Bits = Bits::new(s_5_5 as u128, 1u16);
        // D s_5_13: cast zx s_5_11 -> bv
        let s_5_13: Bits = Bits::new(s_5_11 as u128, 1u16);
        // D s_5_14: or s_5_12 s_5_13
        let s_5_14: Bits = ((s_5_12) | (s_5_13));
        // D s_5_15: cast reint s_5_14 -> u8
        let s_5_15: bool = ((s_5_14.value()) != 0);
        // D s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 1u16);
        // D s_5_17: not s_5_16
        let s_5_17: Bits = !s_5_16;
        // D s_5_18: cast reint s_5_17 -> u8
        let s_5_18: bool = ((s_5_17.value()) != 0);
        // D s_5_19: write-var x <= s_5_18
        fn_state.x = s_5_18;
        // N s_5_20: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var accdesc.1:struct
        let s_6_0: u32 = fn_state.accdesc._1;
        // C s_6_1: const #0u : u32
        let s_6_1: u32 = 0;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // N s_6_3: branch s_6_2 b16 b7
        if s_6_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var accdesc.1:struct
        let s_7_0: u32 = fn_state.accdesc._1;
        // C s_7_1: const #5u : u32
        let s_7_1: u32 = 5;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b15 b8
        if s_7_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var accdesc.1:struct
        let s_8_0: u32 = fn_state.accdesc._1;
        // C s_8_1: const #6u : u32
        let s_8_1: u32 = 6;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: write-var gs#27822 <= s_8_2
        fn_state.gs_27822 = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#27822:u8
        let s_9_0: bool = fn_state.gs_27822;
        // N s_9_1: branch s_9_0 b14 b10
        if s_9_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var accdesc.32:struct
        let s_10_0: bool = fn_state.accdesc._32;
        // N s_10_1: branch s_10_0 b13 b11
        if s_10_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var r:u8
        let s_11_0: bool = fn_state.r;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #0u : u8
        let s_11_2: bool = false;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: write-var return_value <= s_11_4
        fn_state.return_value = s_11_4;
        // N s_11_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var return_value:u8
        let s_12_0: bool = fn_state.return_value;
        // N s_12_1: return s_12_0
        return s_12_0;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var w:u8
        let s_13_0: bool = fn_state.w;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #0u : u8
        let s_13_2: bool = false;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var return_value <= s_13_4
        fn_state.return_value = s_13_4;
        // N s_13_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#27822 <= s_15_0
        fn_state.gs_27822 = s_15_0;
        // N s_15_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #7u : u32
        let s_16_0: u32 = 7;
        // S s_16_1: call ConstrainUnpredictable(s_16_0)
        let s_16_1: u32 = ConstrainUnpredictable(state, tracer, s_16_0);
        // C s_16_2: const #12u : u32
        let s_16_2: u32 = 12;
        // S s_16_3: cmp-eq s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) == (s_16_2));
        // N s_16_4: branch s_16_3 b21 b17
        if s_16_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#27823 <= s_17_0
        fn_state.gs_27823 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#27823:u8
        let s_18_0: bool = fn_state.gs_27823;
        // N s_18_1: branch s_18_0 b20 b19
        if s_18_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var x:u8
        let s_19_0: bool = fn_state.x;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #0u : u8
        let s_19_2: bool = false;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var return_value <= s_19_4
        fn_state.return_value = s_19_4;
        // N s_19_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var return_value <= s_20_0
        fn_state.return_value = s_20_0;
        // N s_20_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var memtype:u32
        let s_21_0: u32 = fn_state.memtype;
        // C s_21_1: const #1u : u32
        let s_21_1: u32 = 1;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // D s_21_3: write-var gs#27823 <= s_21_2
        fn_state.gs_27823 = s_21_2;
        // N s_21_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var r:u8
        let s_22_0: bool = fn_state.r;
        // C s_22_1: const #0u : u8
        let s_22_1: bool = false;
        // D s_22_2: create-product struct = ["s_22_0", "s_22_1"]
        let s_22_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_22_0,
            _1: s_22_1,
        };
        // D s_22_3: write-var ga#21458 <= s_22_2
        fn_state.ga_21458 = s_22_2;
        // D s_22_4: read-var ga#21458.0:struct
        let s_22_4: bool = fn_state.ga_21458._0;
        // D s_22_5: read-var ga#21458.1:struct
        let s_22_5: bool = fn_state.ga_21458._1;
        // D s_22_6: write-var r <= s_22_4
        fn_state.r = s_22_4;
        // D s_22_7: write-var w <= s_22_5
        fn_state.w = s_22_5;
        // N s_22_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // C s_23_1: const #0u : u8
        let s_23_1: bool = false;
        // D s_23_2: create-product struct = ["s_23_0", "s_23_1"]
        let s_23_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_23_0,
            _1: s_23_1,
        };
        // D s_23_3: write-var ga#21453 <= s_23_2
        fn_state.ga_21453 = s_23_2;
        // D s_23_4: read-var ga#21453.0:struct
        let s_23_4: bool = fn_state.ga_21453._0;
        // D s_23_5: read-var ga#21453.1:struct
        let s_23_5: bool = fn_state.ga_21453._1;
        // D s_23_6: write-var r <= s_23_4
        fn_state.r = s_23_4;
        // D s_23_7: write-var w <= s_23_5
        fn_state.w = s_23_5;
        // N s_23_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var perms.0:struct
        let s_24_0: u8 = fn_state.perms._0;
        // C s_24_1: const #1s : i
        let s_24_1: i128 = 1;
        // D s_24_2: cast zx s_24_0 -> bv
        let s_24_2: Bits = Bits::new(s_24_0 as u128, 3u16);
        // C s_24_3: const #1s : i64
        let s_24_3: i64 = 1;
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #1s : i
        let s_24_5: i128 = 1;
        // C s_24_6: add s_24_5 s_24_4
        let s_24_6: i128 = (s_24_5 + s_24_4);
        // D s_24_7: bit-extract s_24_2 s_24_1 s_24_6
        let s_24_7: Bits = (Bits::new(
            ((s_24_2) >> (s_24_1)).value(),
            u16::try_from(s_24_6).unwrap(),
        ));
        // D s_24_8: cast reint s_24_7 -> u8
        let s_24_8: u8 = (s_24_7.value() as u8);
        // D s_24_9: write-var ga#21409 <= s_24_8
        fn_state.ga_21409 = s_24_8;
        // D s_24_10: read-var ga#21409:u8
        let s_24_10: u8 = fn_state.ga_21409;
        // D s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 2u16);
        // C s_24_12: const #0u : u8
        let s_24_12: u8 = 0;
        // C s_24_13: cast zx s_24_12 -> bv
        let s_24_13: Bits = Bits::new(s_24_12 as u128, 2u16);
        // D s_24_14: cmp-eq s_24_11 s_24_13
        let s_24_14: bool = ((s_24_11) == (s_24_13));
        // D s_24_15: not s_24_14
        let s_24_15: bool = !s_24_14;
        // N s_24_16: branch s_24_15 b49 b25
        if s_24_15 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // C s_25_1: const #1u : u8
        let s_25_1: bool = true;
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: const #0u : u8
        let s_25_3: bool = false;
        // D s_25_4: create-product struct = ["s_25_0", "s_25_1", "s_25_2", "s_25_3"]
        let s_25_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_25_0,
            _1: s_25_1,
            _2: s_25_2,
            _3: s_25_3,
        };
        // D s_25_5: write-var ga#21410 <= s_25_4
        fn_state.ga_21410 = s_25_4;
        // D s_25_6: read-var ga#21410.0:struct
        let s_25_6: bool = fn_state.ga_21410._0;
        // D s_25_7: read-var ga#21410.1:struct
        let s_25_7: bool = fn_state.ga_21410._1;
        // D s_25_8: read-var ga#21410.2:struct
        let s_25_8: bool = fn_state.ga_21410._2;
        // D s_25_9: read-var ga#21410.3:struct
        let s_25_9: bool = fn_state.ga_21410._3;
        // D s_25_10: write-var pr <= s_25_6
        fn_state.pr = s_25_6;
        // D s_25_11: write-var pw <= s_25_7
        fn_state.pw = s_25_7;
        // D s_25_12: write-var ur <= s_25_8
        fn_state.ur = s_25_8;
        // D s_25_13: write-var uw <= s_25_9
        fn_state.uw = s_25_9;
        // N s_25_14: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var perms.1:struct
        let s_26_0: u8 = fn_state.perms._1;
        // D s_26_1: write-var ga#21414 <= s_26_0
        fn_state.ga_21414 = s_26_0;
        // D s_26_2: read-var ga#21414:u8
        let s_26_2: u8 = fn_state.ga_21414;
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 2u16);
        // C s_26_4: const #0u : u8
        let s_26_4: u8 = 0;
        // C s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 2u16);
        // D s_26_6: cmp-eq s_26_3 s_26_5
        let s_26_6: bool = ((s_26_3) == (s_26_5));
        // D s_26_7: not s_26_6
        let s_26_7: bool = !s_26_6;
        // N s_26_8: branch s_26_7 b44 b27
        if s_26_7 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var pr:u8
        let s_27_0: bool = fn_state.pr;
        // D s_27_1: read-var pw:u8
        let s_27_1: bool = fn_state.pw;
        // D s_27_2: read-var ur:u8
        let s_27_2: bool = fn_state.ur;
        // D s_27_3: read-var uw:u8
        let s_27_3: bool = fn_state.uw;
        // D s_27_4: create-product struct = ["s_27_0", "s_27_1", "s_27_2", "s_27_3"]
        let s_27_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_27_0,
            _1: s_27_1,
            _2: s_27_2,
            _3: s_27_3,
        };
        // D s_27_5: write-var ga#21415 <= s_27_4
        fn_state.ga_21415 = s_27_4;
        // D s_27_6: read-var ga#21415.0:struct
        let s_27_6: bool = fn_state.ga_21415._0;
        // D s_27_7: read-var ga#21415.1:struct
        let s_27_7: bool = fn_state.ga_21415._1;
        // D s_27_8: read-var ga#21415.2:struct
        let s_27_8: bool = fn_state.ga_21415._2;
        // D s_27_9: read-var ga#21415.3:struct
        let s_27_9: bool = fn_state.ga_21415._3;
        // D s_27_10: write-var pr <= s_27_6
        fn_state.pr = s_27_6;
        // D s_27_11: write-var pw <= s_27_7
        fn_state.pw = s_27_7;
        // D s_27_12: write-var ur <= s_27_8
        fn_state.ur = s_27_8;
        // D s_27_13: write-var uw <= s_27_9
        fn_state.uw = s_27_9;
        // N s_27_14: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var perms.17:struct
        let s_28_0: bool = fn_state.perms._17;
        // D s_28_1: read-var perms.18:struct
        let s_28_1: bool = fn_state.perms._18;
        // D s_28_2: cast zx s_28_0 -> bv
        let s_28_2: Bits = Bits::new(s_28_0 as u128, 1u16);
        // D s_28_3: cast zx s_28_1 -> bv
        let s_28_3: Bits = Bits::new(s_28_1 as u128, 1u16);
        // D s_28_4: or s_28_2 s_28_3
        let s_28_4: Bits = ((s_28_2) | (s_28_3));
        // D s_28_5: cast reint s_28_4 -> u8
        let s_28_5: bool = ((s_28_4.value()) != 0);
        // D s_28_6: read-var perms.5:struct
        let s_28_6: bool = fn_state.perms._5;
        // D s_28_7: read-var perms.6:struct
        let s_28_7: bool = fn_state.perms._6;
        // D s_28_8: cast zx s_28_6 -> bv
        let s_28_8: Bits = Bits::new(s_28_6 as u128, 1u16);
        // D s_28_9: cast zx s_28_7 -> bv
        let s_28_9: Bits = Bits::new(s_28_7 as u128, 1u16);
        // D s_28_10: or s_28_8 s_28_9
        let s_28_10: Bits = ((s_28_8) | (s_28_9));
        // D s_28_11: cast reint s_28_10 -> u8
        let s_28_11: bool = ((s_28_10.value()) != 0);
        // D s_28_12: read-var walkparams.39:struct
        let s_28_12: bool = fn_state.walkparams._39;
        // D s_28_13: read-var uw:u8
        let s_28_13: bool = fn_state.uw;
        // D s_28_14: cast zx s_28_13 -> bv
        let s_28_14: Bits = Bits::new(s_28_13 as u128, 1u16);
        // D s_28_15: cast zx s_28_12 -> bv
        let s_28_15: Bits = Bits::new(s_28_12 as u128, 1u16);
        // D s_28_16: and s_28_14 s_28_15
        let s_28_16: Bits = ((s_28_14) & (s_28_15));
        // D s_28_17: cast reint s_28_16 -> u8
        let s_28_17: bool = ((s_28_16.value()) != 0);
        // D s_28_18: cast zx s_28_5 -> bv
        let s_28_18: Bits = Bits::new(s_28_5 as u128, 1u16);
        // D s_28_19: cast zx s_28_17 -> bv
        let s_28_19: Bits = Bits::new(s_28_17 as u128, 1u16);
        // D s_28_20: or s_28_18 s_28_19
        let s_28_20: Bits = ((s_28_18) | (s_28_19));
        // D s_28_21: cast reint s_28_20 -> u8
        let s_28_21: bool = ((s_28_20.value()) != 0);
        // D s_28_22: cast zx s_28_21 -> bv
        let s_28_22: Bits = Bits::new(s_28_21 as u128, 1u16);
        // D s_28_23: not s_28_22
        let s_28_23: Bits = !s_28_22;
        // D s_28_24: cast reint s_28_23 -> u8
        let s_28_24: bool = ((s_28_23.value()) != 0);
        // D s_28_25: read-var ur:u8
        let s_28_25: bool = fn_state.ur;
        // D s_28_26: cast zx s_28_25 -> bv
        let s_28_26: Bits = Bits::new(s_28_25 as u128, 1u16);
        // D s_28_27: cast zx s_28_24 -> bv
        let s_28_27: Bits = Bits::new(s_28_24 as u128, 1u16);
        // D s_28_28: and s_28_26 s_28_27
        let s_28_28: Bits = ((s_28_26) & (s_28_27));
        // D s_28_29: cast reint s_28_28 -> u8
        let s_28_29: bool = ((s_28_28.value()) != 0);
        // D s_28_30: write-var uxshadow#512 <= s_28_29
        fn_state.uxshadow_512 = s_28_29;
        // D s_28_31: cast zx s_28_5 -> bv
        let s_28_31: Bits = Bits::new(s_28_5 as u128, 1u16);
        // D s_28_32: cast zx s_28_11 -> bv
        let s_28_32: Bits = Bits::new(s_28_11 as u128, 1u16);
        // D s_28_33: or s_28_31 s_28_32
        let s_28_33: Bits = ((s_28_31) | (s_28_32));
        // D s_28_34: cast reint s_28_33 -> u8
        let s_28_34: bool = ((s_28_33.value()) != 0);
        // D s_28_35: read-var walkparams.39:struct
        let s_28_35: bool = fn_state.walkparams._39;
        // D s_28_36: read-var pw:u8
        let s_28_36: bool = fn_state.pw;
        // D s_28_37: cast zx s_28_36 -> bv
        let s_28_37: Bits = Bits::new(s_28_36 as u128, 1u16);
        // D s_28_38: cast zx s_28_35 -> bv
        let s_28_38: Bits = Bits::new(s_28_35 as u128, 1u16);
        // D s_28_39: and s_28_37 s_28_38
        let s_28_39: Bits = ((s_28_37) & (s_28_38));
        // D s_28_40: cast reint s_28_39 -> u8
        let s_28_40: bool = ((s_28_39.value()) != 0);
        // D s_28_41: cast zx s_28_34 -> bv
        let s_28_41: Bits = Bits::new(s_28_34 as u128, 1u16);
        // D s_28_42: cast zx s_28_40 -> bv
        let s_28_42: Bits = Bits::new(s_28_40 as u128, 1u16);
        // D s_28_43: or s_28_41 s_28_42
        let s_28_43: Bits = ((s_28_41) | (s_28_42));
        // D s_28_44: cast reint s_28_43 -> u8
        let s_28_44: bool = ((s_28_43.value()) != 0);
        // D s_28_45: read-var walkparams.38:struct
        let s_28_45: bool = fn_state.walkparams._38;
        // D s_28_46: read-var uw:u8
        let s_28_46: bool = fn_state.uw;
        // D s_28_47: cast zx s_28_46 -> bv
        let s_28_47: Bits = Bits::new(s_28_46 as u128, 1u16);
        // D s_28_48: cast zx s_28_45 -> bv
        let s_28_48: Bits = Bits::new(s_28_45 as u128, 1u16);
        // D s_28_49: and s_28_47 s_28_48
        let s_28_49: Bits = ((s_28_47) & (s_28_48));
        // D s_28_50: cast reint s_28_49 -> u8
        let s_28_50: bool = ((s_28_49.value()) != 0);
        // D s_28_51: cast zx s_28_44 -> bv
        let s_28_51: Bits = Bits::new(s_28_44 as u128, 1u16);
        // D s_28_52: cast zx s_28_50 -> bv
        let s_28_52: Bits = Bits::new(s_28_50 as u128, 1u16);
        // D s_28_53: or s_28_51 s_28_52
        let s_28_53: Bits = ((s_28_51) | (s_28_52));
        // D s_28_54: cast reint s_28_53 -> u8
        let s_28_54: bool = ((s_28_53.value()) != 0);
        // D s_28_55: cast zx s_28_54 -> bv
        let s_28_55: Bits = Bits::new(s_28_54 as u128, 1u16);
        // D s_28_56: not s_28_55
        let s_28_56: Bits = !s_28_55;
        // D s_28_57: cast reint s_28_56 -> u8
        let s_28_57: bool = ((s_28_56.value()) != 0);
        // D s_28_58: read-var pr:u8
        let s_28_58: bool = fn_state.pr;
        // D s_28_59: cast zx s_28_58 -> bv
        let s_28_59: Bits = Bits::new(s_28_58 as u128, 1u16);
        // D s_28_60: cast zx s_28_57 -> bv
        let s_28_60: Bits = Bits::new(s_28_57 as u128, 1u16);
        // D s_28_61: and s_28_59 s_28_60
        let s_28_61: Bits = ((s_28_59) & (s_28_60));
        // D s_28_62: cast reint s_28_61 -> u8
        let s_28_62: bool = ((s_28_61.value()) != 0);
        // D s_28_63: write-var pxshadow#513 <= s_28_62
        fn_state.pxshadow_513 = s_28_62;
        // C s_28_64: const #() : ()
        let s_28_64: () = ();
        // S s_28_65: call HavePANExt(s_28_64)
        let s_28_65: bool = HavePANExt(state, tracer, s_28_64);
        // N s_28_66: branch s_28_65 b43 b29
        if s_28_65 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#27810 <= s_29_0
        fn_state.gs_27810 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var gs#27810:u8
        let s_30_0: bool = fn_state.gs_27810;
        // N s_30_1: branch s_30_0 b42 b31
        if s_30_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var accdesc.8:struct
        let s_32_0: u8 = fn_state.accdesc._8;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 2u16);
        // C s_32_2: const #448u : u32
        let s_32_2: u32 = 448;
        // D s_32_3: read-reg s_32_2:u8
        let s_32_3: u8 = {
            let value = state.read_register::<u8>(s_32_2 as isize);
            tracer.read_register(s_32_2 as isize, value);
            value
        };
        // D s_32_4: cast zx s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 2u16);
        // D s_32_5: cmp-eq s_32_1 s_32_4
        let s_32_5: bool = ((s_32_1) == (s_32_4));
        // N s_32_6: branch s_32_5 b41 b33
        if s_32_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var pr:u8
        let s_33_0: bool = fn_state.pr;
        // D s_33_1: read-var pw:u8
        let s_33_1: bool = fn_state.pw;
        // D s_33_2: read-var pxshadow#513:u8
        let s_33_2: bool = fn_state.pxshadow_513;
        // D s_33_3: create-product struct = ["s_33_0", "s_33_1", "s_33_2"]
        let s_33_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_33_0,
            _1: s_33_1,
            _2: s_33_2,
        };
        // D s_33_4: write-var ga#21427 <= s_33_3
        fn_state.ga_21427 = s_33_3;
        // N s_33_5: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_34_0: read-var ga#21427.0:struct
        let s_34_0: bool = fn_state.ga_21427._0;
        // D s_34_1: read-var ga#21427.1:struct
        let s_34_1: bool = fn_state.ga_21427._1;
        // D s_34_2: read-var ga#21427.2:struct
        let s_34_2: bool = fn_state.ga_21427._2;
        // D s_34_3: write-var r <= s_34_0
        fn_state.r = s_34_0;
        // D s_34_4: write-var w <= s_34_1
        fn_state.w = s_34_1;
        // D s_34_5: write-var x <= s_34_2
        fn_state.x = s_34_2;
        // D s_34_6: read-var accdesc.25:struct
        let s_34_6: u32 = fn_state.accdesc._25;
        // C s_34_7: const #3u : u32
        let s_34_7: u32 = 3;
        // D s_34_8: cmp-eq s_34_6 s_34_7
        let s_34_8: bool = ((s_34_6) == (s_34_7));
        // N s_34_9: branch s_34_8 b40 b35
        if s_34_8 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#27817 <= s_35_0
        fn_state.gs_27817 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var gs#27817:u8
        let s_36_0: bool = fn_state.gs_27817;
        // N s_36_1: branch s_36_0 b39 b37
        if s_36_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_38_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_39_0: read-var walkparams.30:struct
        let s_39_0: bool = fn_state.walkparams._30;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // D s_39_2: not s_39_1
        let s_39_2: Bits = !s_39_1;
        // D s_39_3: cast reint s_39_2 -> u8
        let s_39_3: bool = ((s_39_2.value()) != 0);
        // D s_39_4: read-var x:u8
        let s_39_4: bool = fn_state.x;
        // D s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // D s_39_6: cast zx s_39_3 -> bv
        let s_39_6: Bits = Bits::new(s_39_3 as u128, 1u16);
        // D s_39_7: and s_39_5 s_39_6
        let s_39_7: Bits = ((s_39_5) & (s_39_6));
        // D s_39_8: cast reint s_39_7 -> u8
        let s_39_8: bool = ((s_39_7.value()) != 0);
        // D s_39_9: write-var x <= s_39_8
        fn_state.x = s_39_8;
        // N s_39_10: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_40_0: read-var paspace:u32
        let s_40_0: u32 = fn_state.paspace;
        // C s_40_1: const #0u : u32
        let s_40_1: u32 = 0;
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // D s_40_3: write-var gs#27817 <= s_40_2
        fn_state.gs_27817 = s_40_2;
        // N s_40_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_41_0: read-var ur:u8
        let s_41_0: bool = fn_state.ur;
        // D s_41_1: read-var uw:u8
        let s_41_1: bool = fn_state.uw;
        // D s_41_2: read-var uxshadow#512:u8
        let s_41_2: bool = fn_state.uxshadow_512;
        // D s_41_3: create-product struct = ["s_41_0", "s_41_1", "s_41_2"]
        let s_41_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_41_0,
            _1: s_41_1,
            _2: s_41_2,
        };
        // D s_41_4: write-var ga#21427 <= s_41_3
        fn_state.ga_21427 = s_41_3;
        // N s_41_5: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_42_0: const #16985u : u32
        let s_42_0: u32 = 16985;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: bool = {
            let value = state.read_register::<bool>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: read-var ur:u8
        let s_42_2: bool = fn_state.ur;
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: read-var uw:u8
        let s_42_4: bool = fn_state.uw;
        // D s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // D s_42_6: or s_42_3 s_42_5
        let s_42_6: Bits = ((s_42_3) | (s_42_5));
        // D s_42_7: cast reint s_42_6 -> u8
        let s_42_7: bool = ((s_42_6.value()) != 0);
        // D s_42_8: cast zx s_42_1 -> bv
        let s_42_8: Bits = Bits::new(s_42_1 as u128, 1u16);
        // D s_42_9: cast zx s_42_7 -> bv
        let s_42_9: Bits = Bits::new(s_42_7 as u128, 1u16);
        // D s_42_10: and s_42_8 s_42_9
        let s_42_10: Bits = ((s_42_8) & (s_42_9));
        // D s_42_11: cast reint s_42_10 -> u8
        let s_42_11: bool = ((s_42_10.value()) != 0);
        // D s_42_12: cast zx s_42_11 -> bv
        let s_42_12: Bits = Bits::new(s_42_11 as u128, 1u16);
        // D s_42_13: not s_42_12
        let s_42_13: Bits = !s_42_12;
        // D s_42_14: cast reint s_42_13 -> u8
        let s_42_14: bool = ((s_42_13.value()) != 0);
        // D s_42_15: read-var pr:u8
        let s_42_15: bool = fn_state.pr;
        // D s_42_16: cast zx s_42_15 -> bv
        let s_42_16: Bits = Bits::new(s_42_15 as u128, 1u16);
        // D s_42_17: cast zx s_42_14 -> bv
        let s_42_17: Bits = Bits::new(s_42_14 as u128, 1u16);
        // D s_42_18: and s_42_16 s_42_17
        let s_42_18: Bits = ((s_42_16) & (s_42_17));
        // D s_42_19: cast reint s_42_18 -> u8
        let s_42_19: bool = ((s_42_18.value()) != 0);
        // D s_42_20: write-var pr <= s_42_19
        fn_state.pr = s_42_19;
        // D s_42_21: cast zx s_42_11 -> bv
        let s_42_21: Bits = Bits::new(s_42_11 as u128, 1u16);
        // D s_42_22: not s_42_21
        let s_42_22: Bits = !s_42_21;
        // D s_42_23: cast reint s_42_22 -> u8
        let s_42_23: bool = ((s_42_22.value()) != 0);
        // D s_42_24: read-var pw:u8
        let s_42_24: bool = fn_state.pw;
        // D s_42_25: cast zx s_42_24 -> bv
        let s_42_25: Bits = Bits::new(s_42_24 as u128, 1u16);
        // D s_42_26: cast zx s_42_23 -> bv
        let s_42_26: Bits = Bits::new(s_42_23 as u128, 1u16);
        // D s_42_27: and s_42_25 s_42_26
        let s_42_27: Bits = ((s_42_25) & (s_42_26));
        // D s_42_28: cast reint s_42_27 -> u8
        let s_42_28: bool = ((s_42_27.value()) != 0);
        // D s_42_29: write-var pw <= s_42_28
        fn_state.pw = s_42_28;
        // N s_42_30: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_43_0: read-var accdesc.20:struct
        let s_43_0: bool = fn_state.accdesc._20;
        // D s_43_1: write-var gs#27810 <= s_43_0
        fn_state.gs_27810 = s_43_0;
        // N s_43_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_44_0: read-var ga#21414:u8
        let s_44_0: u8 = fn_state.ga_21414;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 2u16);
        // C s_44_2: const #1u : u8
        let s_44_2: u8 = 1;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 2u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: not s_44_4
        let s_44_5: bool = !s_44_4;
        // N s_44_6: branch s_44_5 b46 b45
        if s_44_5 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_45_0: read-var pr:u8
        let s_45_0: bool = fn_state.pr;
        // D s_45_1: read-var pw:u8
        let s_45_1: bool = fn_state.pw;
        // C s_45_2: const #0u : u8
        let s_45_2: bool = false;
        // C s_45_3: const #0u : u8
        let s_45_3: bool = false;
        // D s_45_4: create-product struct = ["s_45_0", "s_45_1", "s_45_2", "s_45_3"]
        let s_45_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_45_0,
            _1: s_45_1,
            _2: s_45_2,
            _3: s_45_3,
        };
        // D s_45_5: write-var ga#21416 <= s_45_4
        fn_state.ga_21416 = s_45_4;
        // D s_45_6: read-var ga#21416.0:struct
        let s_45_6: bool = fn_state.ga_21416._0;
        // D s_45_7: read-var ga#21416.1:struct
        let s_45_7: bool = fn_state.ga_21416._1;
        // D s_45_8: read-var ga#21416.2:struct
        let s_45_8: bool = fn_state.ga_21416._2;
        // D s_45_9: read-var ga#21416.3:struct
        let s_45_9: bool = fn_state.ga_21416._3;
        // D s_45_10: write-var pr <= s_45_6
        fn_state.pr = s_45_6;
        // D s_45_11: write-var pw <= s_45_7
        fn_state.pw = s_45_7;
        // D s_45_12: write-var ur <= s_45_8
        fn_state.ur = s_45_8;
        // D s_45_13: write-var uw <= s_45_9
        fn_state.uw = s_45_9;
        // N s_45_14: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_46_0: read-var ga#21414:u8
        let s_46_0: u8 = fn_state.ga_21414;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 2u16);
        // C s_46_2: const #2u : u8
        let s_46_2: u8 = 2;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 2u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: not s_46_4
        let s_46_5: bool = !s_46_4;
        // N s_46_6: branch s_46_5 b48 b47
        if s_46_5 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_47_0: read-var pr:u8
        let s_47_0: bool = fn_state.pr;
        // C s_47_1: const #0u : u8
        let s_47_1: bool = false;
        // D s_47_2: read-var ur:u8
        let s_47_2: bool = fn_state.ur;
        // C s_47_3: const #0u : u8
        let s_47_3: bool = false;
        // D s_47_4: create-product struct = ["s_47_0", "s_47_1", "s_47_2", "s_47_3"]
        let s_47_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_47_0,
            _1: s_47_1,
            _2: s_47_2,
            _3: s_47_3,
        };
        // D s_47_5: write-var ga#21417 <= s_47_4
        fn_state.ga_21417 = s_47_4;
        // D s_47_6: read-var ga#21417.0:struct
        let s_47_6: bool = fn_state.ga_21417._0;
        // D s_47_7: read-var ga#21417.1:struct
        let s_47_7: bool = fn_state.ga_21417._1;
        // D s_47_8: read-var ga#21417.2:struct
        let s_47_8: bool = fn_state.ga_21417._2;
        // D s_47_9: read-var ga#21417.3:struct
        let s_47_9: bool = fn_state.ga_21417._3;
        // D s_47_10: write-var pr <= s_47_6
        fn_state.pr = s_47_6;
        // D s_47_11: write-var pw <= s_47_7
        fn_state.pw = s_47_7;
        // D s_47_12: write-var ur <= s_47_8
        fn_state.ur = s_47_8;
        // D s_47_13: write-var uw <= s_47_9
        fn_state.uw = s_47_9;
        // N s_47_14: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_48_0: read-var pr:u8
        let s_48_0: bool = fn_state.pr;
        // C s_48_1: const #0u : u8
        let s_48_1: bool = false;
        // C s_48_2: const #0u : u8
        let s_48_2: bool = false;
        // C s_48_3: const #0u : u8
        let s_48_3: bool = false;
        // D s_48_4: create-product struct = ["s_48_0", "s_48_1", "s_48_2", "s_48_3"]
        let s_48_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_48_0,
            _1: s_48_1,
            _2: s_48_2,
            _3: s_48_3,
        };
        // D s_48_5: write-var ga#21418 <= s_48_4
        fn_state.ga_21418 = s_48_4;
        // D s_48_6: read-var ga#21418.0:struct
        let s_48_6: bool = fn_state.ga_21418._0;
        // D s_48_7: read-var ga#21418.1:struct
        let s_48_7: bool = fn_state.ga_21418._1;
        // D s_48_8: read-var ga#21418.2:struct
        let s_48_8: bool = fn_state.ga_21418._2;
        // D s_48_9: read-var ga#21418.3:struct
        let s_48_9: bool = fn_state.ga_21418._3;
        // D s_48_10: write-var pr <= s_48_6
        fn_state.pr = s_48_6;
        // D s_48_11: write-var pw <= s_48_7
        fn_state.pw = s_48_7;
        // D s_48_12: write-var ur <= s_48_8
        fn_state.ur = s_48_8;
        // D s_48_13: write-var uw <= s_48_9
        fn_state.uw = s_48_9;
        // N s_48_14: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_49_0: read-var ga#21409:u8
        let s_49_0: u8 = fn_state.ga_21409;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 2u16);
        // C s_49_2: const #1u : u8
        let s_49_2: u8 = 1;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 2u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: not s_49_4
        let s_49_5: bool = !s_49_4;
        // N s_49_6: branch s_49_5 b51 b50
        if s_49_5 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // C s_50_1: const #1u : u8
        let s_50_1: bool = true;
        // C s_50_2: const #1u : u8
        let s_50_2: bool = true;
        // C s_50_3: const #1u : u8
        let s_50_3: bool = true;
        // D s_50_4: create-product struct = ["s_50_0", "s_50_1", "s_50_2", "s_50_3"]
        let s_50_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_50_0,
            _1: s_50_1,
            _2: s_50_2,
            _3: s_50_3,
        };
        // D s_50_5: write-var ga#21411 <= s_50_4
        fn_state.ga_21411 = s_50_4;
        // D s_50_6: read-var ga#21411.0:struct
        let s_50_6: bool = fn_state.ga_21411._0;
        // D s_50_7: read-var ga#21411.1:struct
        let s_50_7: bool = fn_state.ga_21411._1;
        // D s_50_8: read-var ga#21411.2:struct
        let s_50_8: bool = fn_state.ga_21411._2;
        // D s_50_9: read-var ga#21411.3:struct
        let s_50_9: bool = fn_state.ga_21411._3;
        // D s_50_10: write-var pr <= s_50_6
        fn_state.pr = s_50_6;
        // D s_50_11: write-var pw <= s_50_7
        fn_state.pw = s_50_7;
        // D s_50_12: write-var ur <= s_50_8
        fn_state.ur = s_50_8;
        // D s_50_13: write-var uw <= s_50_9
        fn_state.uw = s_50_9;
        // N s_50_14: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_51_0: read-var ga#21409:u8
        let s_51_0: u8 = fn_state.ga_21409;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 2u16);
        // C s_51_2: const #2u : u8
        let s_51_2: u8 = 2;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 2u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: not s_51_4
        let s_51_5: bool = !s_51_4;
        // N s_51_6: branch s_51_5 b53 b52
        if s_51_5 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // C s_52_1: const #0u : u8
        let s_52_1: bool = false;
        // C s_52_2: const #0u : u8
        let s_52_2: bool = false;
        // C s_52_3: const #0u : u8
        let s_52_3: bool = false;
        // D s_52_4: create-product struct = ["s_52_0", "s_52_1", "s_52_2", "s_52_3"]
        let s_52_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_52_0,
            _1: s_52_1,
            _2: s_52_2,
            _3: s_52_3,
        };
        // D s_52_5: write-var ga#21412 <= s_52_4
        fn_state.ga_21412 = s_52_4;
        // D s_52_6: read-var ga#21412.0:struct
        let s_52_6: bool = fn_state.ga_21412._0;
        // D s_52_7: read-var ga#21412.1:struct
        let s_52_7: bool = fn_state.ga_21412._1;
        // D s_52_8: read-var ga#21412.2:struct
        let s_52_8: bool = fn_state.ga_21412._2;
        // D s_52_9: read-var ga#21412.3:struct
        let s_52_9: bool = fn_state.ga_21412._3;
        // D s_52_10: write-var pr <= s_52_6
        fn_state.pr = s_52_6;
        // D s_52_11: write-var pw <= s_52_7
        fn_state.pw = s_52_7;
        // D s_52_12: write-var ur <= s_52_8
        fn_state.ur = s_52_8;
        // D s_52_13: write-var uw <= s_52_9
        fn_state.uw = s_52_9;
        // N s_52_14: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // C s_53_1: const #0u : u8
        let s_53_1: bool = false;
        // C s_53_2: const #1u : u8
        let s_53_2: bool = true;
        // C s_53_3: const #0u : u8
        let s_53_3: bool = false;
        // D s_53_4: create-product struct = ["s_53_0", "s_53_1", "s_53_2", "s_53_3"]
        let s_53_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_53_0,
            _1: s_53_1,
            _2: s_53_2,
            _3: s_53_3,
        };
        // D s_53_5: write-var ga#21413 <= s_53_4
        fn_state.ga_21413 = s_53_4;
        // D s_53_6: read-var ga#21413.0:struct
        let s_53_6: bool = fn_state.ga_21413._0;
        // D s_53_7: read-var ga#21413.1:struct
        let s_53_7: bool = fn_state.ga_21413._1;
        // D s_53_8: read-var ga#21413.2:struct
        let s_53_8: bool = fn_state.ga_21413._2;
        // D s_53_9: read-var ga#21413.3:struct
        let s_53_9: bool = fn_state.ga_21413._3;
        // D s_53_10: write-var pr <= s_53_6
        fn_state.pr = s_53_6;
        // D s_53_11: write-var pw <= s_53_7
        fn_state.pw = s_53_7;
        // D s_53_12: write-var ur <= s_53_8
        fn_state.ur = s_53_8;
        // D s_53_13: write-var uw <= s_53_9
        fn_state.uw = s_53_9;
        // N s_53_14: jump b26
        return block_26(state, tracer, fn_state);
    }
}
