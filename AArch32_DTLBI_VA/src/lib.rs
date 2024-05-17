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
use TLBI::*;
use Zeros::*;
use common::*;
pub fn AArch32_DTLBI_VA<T: Tracer>(
    state: &mut State,
    tracer: &T,
    security: u32,
    regime: u32,
    vmid: u16,
    shareability: u32,
    level: u32,
    attr: u32,
    Rt: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductTypefb7b2cabacce34a2,
        gs_32328: bool,
        gs_32327: bool,
        security: u32,
        regime: u32,
        vmid: u16,
        shareability: u32,
        level: u32,
        attr: u32,
        Rt: u32,
    }
    let fn_state = FunctionState {
        security,
        regime,
        vmid,
        shareability,
        level,
        attr,
        Rt,
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
        // N s_0_7: branch s_0_6 b8 b1
        if s_0_6 {
            return block_8(state, tracer, fn_state);
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
        // N s_1_7: branch s_1_6 b7 b2
        if s_1_6 {
            return block_7(state, tracer, fn_state);
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
        // D s_2_7: write-var gs#32327 <= s_2_6
        fn_state.gs_32327 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#32327:u8
        let s_3_0: bool = fn_state.gs_32327;
        // D s_3_1: write-var gs#32328 <= s_3_0
        fn_state.gs_32328 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#32328:u8
        let s_4_0: bool = fn_state.gs_32328;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #2u : u32
        let s_4_2: u32 = 2;
        // D s_4_3: write-var r.9 <= s_4_2
        fn_state.r._9 = s_4_2;
        // C s_4_4: const #0u : u8
        let s_4_4: bool = false;
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
        // C s_4_16: const #8s : i
        let s_4_16: i128 = 8;
        // S s_4_17: call Zeros(s_4_16)
        let s_4_17: Bits = Zeros(state, tracer, s_4_16);
        // S s_4_18: cast reint s_4_17 -> u8
        let s_4_18: u8 = (s_4_17.value() as u8);
        // C s_4_19: const #0s : i
        let s_4_19: i128 = 0;
        // D s_4_20: read-var Rt:u32
        let s_4_20: u32 = fn_state.Rt;
        // D s_4_21: cast zx s_4_20 -> bv
        let s_4_21: Bits = Bits::new(s_4_20 as u128, 32u16);
        // C s_4_22: const #1s : i64
        let s_4_22: i64 = 1;
        // C s_4_23: cast zx s_4_22 -> i
        let s_4_23: i128 = (i128::try_from(s_4_22).unwrap());
        // C s_4_24: const #7s : i
        let s_4_24: i128 = 7;
        // C s_4_25: add s_4_24 s_4_23
        let s_4_25: i128 = (s_4_24 + s_4_23);
        // D s_4_26: bit-extract s_4_21 s_4_19 s_4_25
        let s_4_26: Bits = (Bits::new(
            ((s_4_21) >> (s_4_19)).value(),
            u16::try_from(s_4_25).unwrap(),
        ));
        // D s_4_27: cast reint s_4_26 -> u8
        let s_4_27: u8 = (s_4_26.value() as u8);
        // S s_4_28: cast zx s_4_18 -> bv
        let s_4_28: Bits = Bits::new(s_4_18 as u128, 8u16);
        // D s_4_29: cast zx s_4_27 -> bv
        let s_4_29: Bits = Bits::new(s_4_27 as u128, 8u16);
        // S s_4_30: cast reint s_4_28 -> u128
        let s_4_30: u128 = (s_4_28.value() as u128);
        // D s_4_31: size-of s_4_28
        let s_4_31: u16 = s_4_28.length();
        // D s_4_32: cast reint s_4_29 -> u128
        let s_4_32: u128 = (s_4_29.value() as u128);
        // D s_4_33: size-of s_4_29
        let s_4_33: u16 = s_4_29.length();
        // D s_4_34: lsl s_4_30 s_4_33
        let s_4_34: u128 = s_4_30 << s_4_33;
        // D s_4_35: or s_4_34 s_4_32
        let s_4_35: u128 = ((s_4_34) | (s_4_32));
        // D s_4_36: add s_4_31 s_4_33
        let s_4_36: u16 = (s_4_31 + s_4_33);
        // D s_4_37: create-bits s_4_35 s_4_36
        let s_4_37: Bits = Bits::new(s_4_35, s_4_36);
        // D s_4_38: cast reint s_4_37 -> u16
        let s_4_38: u16 = (s_4_37.value() as u16);
        // D s_4_39: write-var r.1 <= s_4_38
        fn_state.r._1 = s_4_38;
        // C s_4_40: const #32s : i
        let s_4_40: i128 = 32;
        // S s_4_41: call Zeros(s_4_40)
        let s_4_41: Bits = Zeros(state, tracer, s_4_40);
        // S s_4_42: cast reint s_4_41 -> u32
        let s_4_42: u32 = (s_4_41.value() as u32);
        // C s_4_43: const #12s : i
        let s_4_43: i128 = 12;
        // D s_4_44: read-var Rt:u32
        let s_4_44: u32 = fn_state.Rt;
        // D s_4_45: cast zx s_4_44 -> bv
        let s_4_45: Bits = Bits::new(s_4_44 as u128, 32u16);
        // C s_4_46: const #1s : i64
        let s_4_46: i64 = 1;
        // C s_4_47: cast zx s_4_46 -> i
        let s_4_47: i128 = (i128::try_from(s_4_46).unwrap());
        // C s_4_48: const #19s : i
        let s_4_48: i128 = 19;
        // C s_4_49: add s_4_48 s_4_47
        let s_4_49: i128 = (s_4_48 + s_4_47);
        // D s_4_50: bit-extract s_4_45 s_4_43 s_4_49
        let s_4_50: Bits = (Bits::new(
            ((s_4_45) >> (s_4_43)).value(),
            u16::try_from(s_4_49).unwrap(),
        ));
        // D s_4_51: cast reint s_4_50 -> u20
        let s_4_51: u32 = (s_4_50.value() as u32);
        // S s_4_52: cast zx s_4_42 -> bv
        let s_4_52: Bits = Bits::new(s_4_42 as u128, 32u16);
        // D s_4_53: cast zx s_4_51 -> bv
        let s_4_53: Bits = Bits::new(s_4_51 as u128, 20u16);
        // S s_4_54: cast reint s_4_52 -> u128
        let s_4_54: u128 = (s_4_52.value() as u128);
        // D s_4_55: size-of s_4_52
        let s_4_55: u16 = s_4_52.length();
        // D s_4_56: cast reint s_4_53 -> u128
        let s_4_56: u128 = (s_4_53.value() as u128);
        // D s_4_57: size-of s_4_53
        let s_4_57: u16 = s_4_53.length();
        // D s_4_58: lsl s_4_54 s_4_57
        let s_4_58: u128 = s_4_54 << s_4_57;
        // D s_4_59: or s_4_58 s_4_56
        let s_4_59: u128 = ((s_4_58) | (s_4_56));
        // D s_4_60: add s_4_55 s_4_57
        let s_4_60: u16 = (s_4_55 + s_4_57);
        // D s_4_61: create-bits s_4_59 s_4_60
        let s_4_61: Bits = Bits::new(s_4_59, s_4_60);
        // D s_4_62: cast reint s_4_61 -> u52
        let s_4_62: u64 = (s_4_61.value() as u64);
        // C s_4_63: const #12s : i
        let s_4_63: i128 = 12;
        // S s_4_64: call Zeros(s_4_63)
        let s_4_64: Bits = Zeros(state, tracer, s_4_63);
        // S s_4_65: cast reint s_4_64 -> u12
        let s_4_65: u16 = (s_4_64.value() as u16);
        // D s_4_66: cast zx s_4_62 -> bv
        let s_4_66: Bits = Bits::new(s_4_62 as u128, 52u16);
        // S s_4_67: cast zx s_4_65 -> bv
        let s_4_67: Bits = Bits::new(s_4_65 as u128, 12u16);
        // D s_4_68: cast reint s_4_66 -> u128
        let s_4_68: u128 = (s_4_66.value() as u128);
        // D s_4_69: size-of s_4_66
        let s_4_69: u16 = s_4_66.length();
        // S s_4_70: cast reint s_4_67 -> u128
        let s_4_70: u128 = (s_4_67.value() as u128);
        // D s_4_71: size-of s_4_67
        let s_4_71: u16 = s_4_67.length();
        // D s_4_72: lsl s_4_68 s_4_71
        let s_4_72: u128 = s_4_68 << s_4_71;
        // D s_4_73: or s_4_72 s_4_70
        let s_4_73: u128 = ((s_4_72) | (s_4_70));
        // D s_4_74: add s_4_69 s_4_71
        let s_4_74: u16 = (s_4_69 + s_4_71);
        // D s_4_75: create-bits s_4_73 s_4_74
        let s_4_75: Bits = Bits::new(s_4_73, s_4_74);
        // D s_4_76: cast reint s_4_75 -> u64
        let s_4_76: u64 = (s_4_75.value() as u64);
        // D s_4_77: write-var r.0 <= s_4_76
        fn_state.r._0 = s_4_76;
        // D s_4_78: read-var r:struct
        let s_4_78: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_4_79: read-var shareability:u32
        let s_4_79: u32 = fn_state.shareability;
        // D s_4_80: call TLBI(s_4_78, s_4_79)
        let s_4_80: () = TLBI(state, tracer, s_4_78, s_4_79);
        // D s_4_81: read-var shareability:u32
        let s_4_81: u32 = fn_state.shareability;
        // C s_4_82: const #0u : u32
        let s_4_82: u32 = 0;
        // D s_4_83: cmp-eq s_4_81 s_4_82
        let s_4_83: bool = ((s_4_81) == (s_4_82));
        // N s_4_84: branch s_4_83 b6 b5
        if s_4_83 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var shareability:u32
        let s_6_0: u32 = fn_state.shareability;
        // D s_6_1: read-var r:struct
        let s_6_1: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_6_2: call Broadcast(s_6_0, s_6_1)
        let s_6_2: () = Broadcast(state, tracer, s_6_0, s_6_1);
        // N s_6_3: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#32327 <= s_7_0
        fn_state.gs_32327 = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#32328 <= s_8_0
        fn_state.gs_32328 = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
