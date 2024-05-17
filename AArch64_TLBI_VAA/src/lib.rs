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
use Broadcast::*;
use place_subrange::*;
use TLBI::*;
use common::*;
pub fn AArch64_TLBI_VAA<T: Tracer>(
    state: &mut State,
    tracer: &T,
    security: u32,
    regime: u32,
    vmid: u16,
    shareability: u32,
    level: u32,
    attr: u32,
    Xt: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductTypefb7b2cabacce34a2,
        gs_26608: bool,
        gs_26607: bool,
        gs_26616: bool,
        security: u32,
        regime: u32,
        vmid: u16,
        shareability: u32,
        level: u32,
        attr: u32,
        Xt: u64,
    }
    let fn_state = FunctionState {
        security,
        regime,
        vmid,
        shareability,
        level,
        attr,
        Xt,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #424u : u32
        let s_0_3: u32 = 424;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b11 b1
        if s_0_6 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #432u : u32
        let s_1_3: u32 = 432;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b10 b2
        if s_1_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16975u : u32
        let s_2_0: u32 = 16975;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #440u : u32
        let s_2_3: u32 = 440;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // D s_2_7: write-var gs#26607 <= s_2_6
        fn_state.gs_26607 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#26607:u8
        let s_3_0: bool = fn_state.gs_26607;
        // D s_3_1: write-var gs#26608 <= s_3_0
        fn_state.gs_26608 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#26608:u8
        let s_4_0: bool = fn_state.gs_26608;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #10u : u32
        let s_4_2: u32 = 10;
        // D s_4_3: write-var r.9 <= s_4_2
        fn_state.r._9 = s_4_2;
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // D s_4_5: write-var r.6 <= s_4_4
        fn_state.r._6 = s_4_4;
        // D s_4_6: read-var security:u32
        let s_4_6: u32 = fn_state.security;
        // D s_4_7: write-var r.11 <= s_4_6
        fn_state.r._11 = s_4_6;
        // D s_4_8: read-var regime:u32
        let s_4_8: u32 = fn_state.regime;
        // D s_4_9: write-var r.10 <= s_4_8
        fn_state.r._10 = s_4_8;
        // D s_4_10: read-var vmid:u16
        let s_4_10: u16 = fn_state.vmid;
        // D s_4_11: write-var r.14 <= s_4_10
        fn_state.r._14 = s_4_10;
        // D s_4_12: read-var level:u32
        let s_4_12: u32 = fn_state.level;
        // D s_4_13: write-var r.8 <= s_4_12
        fn_state.r._8 = s_4_12;
        // D s_4_14: read-var attr:u32
        let s_4_14: u32 = fn_state.attr;
        // D s_4_15: write-var r.2 <= s_4_14
        fn_state.r._2 = s_4_14;
        // C s_4_16: const #44s : i
        let s_4_16: i128 = 44;
        // D s_4_17: read-var Xt:u64
        let s_4_17: u64 = fn_state.Xt;
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 64u16);
        // C s_4_19: const #1s : i64
        let s_4_19: i64 = 1;
        // C s_4_20: cast zx s_4_19 -> i
        let s_4_20: i128 = (i128::try_from(s_4_19).unwrap());
        // C s_4_21: const #3s : i
        let s_4_21: i128 = 3;
        // C s_4_22: add s_4_21 s_4_20
        let s_4_22: i128 = (s_4_21 + s_4_20);
        // D s_4_23: bit-extract s_4_18 s_4_16 s_4_22
        let s_4_23: Bits = (Bits::new(
            ((s_4_18) >> (s_4_16)).value(),
            u16::try_from(s_4_22).unwrap(),
        ));
        // D s_4_24: cast reint s_4_23 -> u8
        let s_4_24: u8 = (s_4_23.value() as u8);
        // D s_4_25: write-var r.13 <= s_4_24
        fn_state.r._13 = s_4_24;
        // C s_4_26: const #64s : i
        let s_4_26: i128 = 64;
        // C s_4_27: const #43s : i
        let s_4_27: i128 = 43;
        // C s_4_28: const #0s : i
        let s_4_28: i128 = 0;
        // C s_4_29: const #12s : i
        let s_4_29: i128 = 12;
        // D s_4_30: read-var Xt:u64
        let s_4_30: u64 = fn_state.Xt;
        // D s_4_31: cast zx s_4_30 -> bv
        let s_4_31: Bits = Bits::new(s_4_30 as u128, 64u16);
        // D s_4_32: call place_subrange(s_4_26, s_4_31, s_4_27, s_4_28, s_4_29)
        let s_4_32: Bits = place_subrange(
            state,
            tracer,
            s_4_26,
            s_4_31,
            s_4_27,
            s_4_28,
            s_4_29,
        );
        // D s_4_33: cast reint s_4_32 -> u64
        let s_4_33: u64 = (s_4_32.value() as u64);
        // D s_4_34: write-var r.0 <= s_4_33
        fn_state.r._0 = s_4_33;
        // C s_4_35: const #1u : u8
        let s_4_35: bool = true;
        // D s_4_36: write-var r.4 <= s_4_35
        fn_state.r._4 = s_4_35;
        // D s_4_37: read-var r.13:struct
        let s_4_37: u8 = fn_state.r._13;
        // C s_4_38: const #2s : i
        let s_4_38: i128 = 2;
        // D s_4_39: cast zx s_4_37 -> bv
        let s_4_39: Bits = Bits::new(s_4_37 as u128, 4u16);
        // C s_4_40: const #1s : i64
        let s_4_40: i64 = 1;
        // C s_4_41: cast zx s_4_40 -> i
        let s_4_41: i128 = (i128::try_from(s_4_40).unwrap());
        // C s_4_42: const #1s : i
        let s_4_42: i128 = 1;
        // C s_4_43: add s_4_42 s_4_41
        let s_4_43: i128 = (s_4_42 + s_4_41);
        // D s_4_44: bit-extract s_4_39 s_4_38 s_4_43
        let s_4_44: Bits = (Bits::new(
            ((s_4_39) >> (s_4_38)).value(),
            u16::try_from(s_4_43).unwrap(),
        ));
        // D s_4_45: cast reint s_4_44 -> u8
        let s_4_45: u8 = (s_4_44.value() as u8);
        // D s_4_46: cast zx s_4_45 -> bv
        let s_4_46: Bits = Bits::new(s_4_45 as u128, 2u16);
        // C s_4_47: const #0u : u8
        let s_4_47: u8 = 0;
        // C s_4_48: cast zx s_4_47 -> bv
        let s_4_48: Bits = Bits::new(s_4_47 as u128, 2u16);
        // D s_4_49: cmp-eq s_4_46 s_4_48
        let s_4_49: bool = ((s_4_46) == (s_4_48));
        // D s_4_50: not s_4_49
        let s_4_50: bool = !s_4_49;
        // N s_4_51: branch s_4_50 b9 b5
        if s_4_50 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#26616 <= s_5_0
        fn_state.gs_26616 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#26616:u8
        let s_6_0: bool = fn_state.gs_26616;
        // D s_6_1: write-var r.3 <= s_6_0
        fn_state.r._3 = s_6_0;
        // D s_6_2: read-var r:struct
        let s_6_2: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_6_3: read-var shareability:u32
        let s_6_3: u32 = fn_state.shareability;
        // D s_6_4: call TLBI(s_6_2, s_6_3)
        let s_6_4: () = TLBI(state, tracer, s_6_2, s_6_3);
        // D s_6_5: read-var shareability:u32
        let s_6_5: u32 = fn_state.shareability;
        // C s_6_6: const #0u : u32
        let s_6_6: u32 = 0;
        // D s_6_7: cmp-eq s_6_5 s_6_6
        let s_6_7: bool = ((s_6_5) == (s_6_6));
        // N s_6_8: branch s_6_7 b8 b7
        if s_6_7 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var shareability:u32
        let s_8_0: u32 = fn_state.shareability;
        // D s_8_1: read-var r:struct
        let s_8_1: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_8_2: call Broadcast(s_8_0, s_8_1)
        let s_8_2: () = Broadcast(state, tracer, s_8_0, s_8_1);
        // N s_8_3: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#26616 <= s_9_0
        fn_state.gs_26616 = s_9_0;
        // N s_9_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#26607 <= s_10_0
        fn_state.gs_26607 = s_10_0;
        // N s_10_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#26608 <= s_11_0
        fn_state.gs_26608 = s_11_0;
        // N s_11_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
