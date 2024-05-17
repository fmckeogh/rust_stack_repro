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
pub fn AArch64_TLBIP_VA<T: Tracer>(
    state: &mut State,
    tracer: &T,
    security: u32,
    regime: u32,
    vmid: u16,
    shareability: u32,
    level: u32,
    attr: u32,
    Xt: u128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductTypefb7b2cabacce34a2,
        gs_26577: bool,
        gs_26588: bool,
        gs_26578: bool,
        security: u32,
        regime: u32,
        vmid: u16,
        shareability: u32,
        level: u32,
        attr: u32,
        Xt: u128,
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
        // D s_2_7: write-var gs#26577 <= s_2_6
        fn_state.gs_26577 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#26577:u8
        let s_3_0: bool = fn_state.gs_26577;
        // D s_3_1: write-var gs#26578 <= s_3_0
        fn_state.gs_26578 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#26578:u8
        let s_4_0: bool = fn_state.gs_26578;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #11u : u32
        let s_4_2: u32 = 11;
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
        // C s_4_16: const #48s : i
        let s_4_16: i128 = 48;
        // D s_4_17: read-var Xt:u128
        let s_4_17: u128 = fn_state.Xt;
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 128u16);
        // C s_4_19: const #1s : i64
        let s_4_19: i64 = 1;
        // C s_4_20: cast zx s_4_19 -> i
        let s_4_20: i128 = (i128::try_from(s_4_19).unwrap());
        // C s_4_21: const #15s : i
        let s_4_21: i128 = 15;
        // C s_4_22: add s_4_21 s_4_20
        let s_4_22: i128 = (s_4_21 + s_4_20);
        // D s_4_23: bit-extract s_4_18 s_4_16 s_4_22
        let s_4_23: Bits = (Bits::new(
            ((s_4_18) >> (s_4_16)).value(),
            u16::try_from(s_4_22).unwrap(),
        ));
        // D s_4_24: cast reint s_4_23 -> u16
        let s_4_24: u16 = (s_4_23.value() as u16);
        // D s_4_25: write-var r.1 <= s_4_24
        fn_state.r._1 = s_4_24;
        // C s_4_26: const #44s : i
        let s_4_26: i128 = 44;
        // D s_4_27: read-var Xt:u128
        let s_4_27: u128 = fn_state.Xt;
        // D s_4_28: cast zx s_4_27 -> bv
        let s_4_28: Bits = Bits::new(s_4_27 as u128, 128u16);
        // C s_4_29: const #1s : i64
        let s_4_29: i64 = 1;
        // C s_4_30: cast zx s_4_29 -> i
        let s_4_30: i128 = (i128::try_from(s_4_29).unwrap());
        // C s_4_31: const #3s : i
        let s_4_31: i128 = 3;
        // C s_4_32: add s_4_31 s_4_30
        let s_4_32: i128 = (s_4_31 + s_4_30);
        // D s_4_33: bit-extract s_4_28 s_4_26 s_4_32
        let s_4_33: Bits = (Bits::new(
            ((s_4_28) >> (s_4_26)).value(),
            u16::try_from(s_4_32).unwrap(),
        ));
        // D s_4_34: cast reint s_4_33 -> u8
        let s_4_34: u8 = (s_4_33.value() as u8);
        // D s_4_35: write-var r.13 <= s_4_34
        fn_state.r._13 = s_4_34;
        // C s_4_36: const #64s : i
        let s_4_36: i128 = 64;
        // C s_4_37: const #107s : i
        let s_4_37: i128 = 107;
        // C s_4_38: const #64s : i
        let s_4_38: i128 = 64;
        // C s_4_39: const #12s : i
        let s_4_39: i128 = 12;
        // D s_4_40: read-var Xt:u128
        let s_4_40: u128 = fn_state.Xt;
        // D s_4_41: cast zx s_4_40 -> bv
        let s_4_41: Bits = Bits::new(s_4_40 as u128, 128u16);
        // D s_4_42: call place_subrange(s_4_36, s_4_41, s_4_37, s_4_38, s_4_39)
        let s_4_42: Bits = place_subrange(
            state,
            tracer,
            s_4_36,
            s_4_41,
            s_4_37,
            s_4_38,
            s_4_39,
        );
        // D s_4_43: cast reint s_4_42 -> u64
        let s_4_43: u64 = (s_4_42.value() as u64);
        // D s_4_44: write-var r.0 <= s_4_43
        fn_state.r._0 = s_4_43;
        // D s_4_45: read-var r.13:struct
        let s_4_45: u8 = fn_state.r._13;
        // C s_4_46: const #2s : i
        let s_4_46: i128 = 2;
        // D s_4_47: cast zx s_4_45 -> bv
        let s_4_47: Bits = Bits::new(s_4_45 as u128, 4u16);
        // C s_4_48: const #1s : i64
        let s_4_48: i64 = 1;
        // C s_4_49: cast zx s_4_48 -> i
        let s_4_49: i128 = (i128::try_from(s_4_48).unwrap());
        // C s_4_50: const #1s : i
        let s_4_50: i128 = 1;
        // C s_4_51: add s_4_50 s_4_49
        let s_4_51: i128 = (s_4_50 + s_4_49);
        // D s_4_52: bit-extract s_4_47 s_4_46 s_4_51
        let s_4_52: Bits = (Bits::new(
            ((s_4_47) >> (s_4_46)).value(),
            u16::try_from(s_4_51).unwrap(),
        ));
        // D s_4_53: cast reint s_4_52 -> u8
        let s_4_53: u8 = (s_4_52.value() as u8);
        // D s_4_54: cast zx s_4_53 -> bv
        let s_4_54: Bits = Bits::new(s_4_53 as u128, 2u16);
        // C s_4_55: const #0u : u8
        let s_4_55: u8 = 0;
        // C s_4_56: cast zx s_4_55 -> bv
        let s_4_56: Bits = Bits::new(s_4_55 as u128, 2u16);
        // D s_4_57: cmp-eq s_4_54 s_4_56
        let s_4_57: bool = ((s_4_54) == (s_4_56));
        // D s_4_58: not s_4_57
        let s_4_58: bool = !s_4_57;
        // N s_4_59: branch s_4_58 b9 b5
        if s_4_58 {
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
        // D s_5_1: write-var gs#26588 <= s_5_0
        fn_state.gs_26588 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#26588:u8
        let s_6_0: bool = fn_state.gs_26588;
        // D s_6_1: write-var r.4 <= s_6_0
        fn_state.r._4 = s_6_0;
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // D s_6_3: write-var r.3 <= s_6_2
        fn_state.r._3 = s_6_2;
        // D s_6_4: read-var r:struct
        let s_6_4: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_6_5: read-var shareability:u32
        let s_6_5: u32 = fn_state.shareability;
        // D s_6_6: call TLBI(s_6_4, s_6_5)
        let s_6_6: () = TLBI(state, tracer, s_6_4, s_6_5);
        // D s_6_7: read-var shareability:u32
        let s_6_7: u32 = fn_state.shareability;
        // C s_6_8: const #0u : u32
        let s_6_8: u32 = 0;
        // D s_6_9: cmp-eq s_6_7 s_6_8
        let s_6_9: bool = ((s_6_7) == (s_6_8));
        // N s_6_10: branch s_6_9 b8 b7
        if s_6_9 {
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
        // D s_9_1: write-var gs#26588 <= s_9_0
        fn_state.gs_26588 = s_9_0;
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
        // D s_10_1: write-var gs#26577 <= s_10_0
        fn_state.gs_26577 = s_10_0;
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
        // D s_11_1: write-var gs#26578 <= s_11_0
        fn_state.gs_26578 = s_11_0;
        // N s_11_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
