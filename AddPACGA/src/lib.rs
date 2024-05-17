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
use TrapPACUse::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_API::*;
use u_get_SCR_EL3_Type_API::*;
use Zeros::*;
use EL2Enabled::*;
use EL3SDDUndef::*;
use EL3SDDUndefPriority::*;
use ComputePAC::*;
use common::*;
pub fn AddPACGA<T: Tracer>(state: &mut State, tracer: &T, x: u64, y: u64) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        ga_11349: u64,
        gs_15307: bool,
        gs_15299: bool,
        gs_15303: bool,
        return_value: u64,
        gs_15310: bool,
        gs_15305: bool,
        TrapEL3: bool,
        ga_11331: u8,
        gs_15301: bool,
        TrapEL2: bool,
        APGAKey_EL1: u128,
        gs_15300: bool,
        gs_15316: bool,
        ga_11353: u64,
        x: u64,
        y: u64,
    }
    let fn_state = FunctionState {
        x,
        y,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // C s_0_1: const #13432u : u32
        let s_0_1: u32 = 13432;
        // D s_0_2: read-reg s_0_1:u64
        let s_0_2: u64 = {
            let value = state.read_register::<u64>(s_0_1 as isize);
            tracer.read_register(s_0_1 as isize, value);
            value
        };
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 64u16);
        // C s_0_4: const #1s : i64
        let s_0_4: i64 = 1;
        // C s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // C s_0_6: const #63s : i
        let s_0_6: i128 = 63;
        // C s_0_7: add s_0_6 s_0_5
        let s_0_7: i128 = (s_0_6 + s_0_5);
        // D s_0_8: bit-extract s_0_3 s_0_0 s_0_7
        let s_0_8: Bits = (Bits::new(
            ((s_0_3) >> (s_0_0)).value(),
            u16::try_from(s_0_7).unwrap(),
        ));
        // D s_0_9: cast reint s_0_8 -> u64
        let s_0_9: u64 = (s_0_8.value() as u64);
        // C s_0_10: const #0s : i
        let s_0_10: i128 = 0;
        // C s_0_11: const #11896u : u32
        let s_0_11: u32 = 11896;
        // D s_0_12: read-reg s_0_11:u64
        let s_0_12: u64 = {
            let value = state.read_register::<u64>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 64u16);
        // C s_0_14: const #1s : i64
        let s_0_14: i64 = 1;
        // C s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // C s_0_16: const #63s : i
        let s_0_16: i128 = 63;
        // C s_0_17: add s_0_16 s_0_15
        let s_0_17: i128 = (s_0_16 + s_0_15);
        // D s_0_18: bit-extract s_0_13 s_0_10 s_0_17
        let s_0_18: Bits = (Bits::new(
            ((s_0_13) >> (s_0_10)).value(),
            u16::try_from(s_0_17).unwrap(),
        ));
        // D s_0_19: cast reint s_0_18 -> u64
        let s_0_19: u64 = (s_0_18.value() as u64);
        // D s_0_20: cast zx s_0_9 -> bv
        let s_0_20: Bits = Bits::new(s_0_9 as u128, 64u16);
        // D s_0_21: cast zx s_0_19 -> bv
        let s_0_21: Bits = Bits::new(s_0_19 as u128, 64u16);
        // D s_0_22: cast reint s_0_20 -> u128
        let s_0_22: u128 = (s_0_20.value() as u128);
        // D s_0_23: size-of s_0_20
        let s_0_23: u16 = s_0_20.length();
        // D s_0_24: cast reint s_0_21 -> u128
        let s_0_24: u128 = (s_0_21.value() as u128);
        // D s_0_25: size-of s_0_21
        let s_0_25: u16 = s_0_21.length();
        // D s_0_26: lsl s_0_22 s_0_25
        let s_0_26: u128 = s_0_22 << s_0_25;
        // D s_0_27: or s_0_26 s_0_24
        let s_0_27: u128 = ((s_0_26) | (s_0_24));
        // D s_0_28: add s_0_23 s_0_25
        let s_0_28: u16 = (s_0_23 + s_0_25);
        // D s_0_29: create-bits s_0_27 s_0_28
        let s_0_29: Bits = Bits::new(s_0_27, s_0_28);
        // D s_0_30: cast reint s_0_29 -> u128
        let s_0_30: u128 = (s_0_29.value() as u128);
        // D s_0_31: write-var APGAKey_EL1 <= s_0_30
        fn_state.APGAKey_EL1 = s_0_30;
        // C s_0_32: const #16975u : u32
        let s_0_32: u32 = 16975;
        // D s_0_33: read-reg s_0_32:u8
        let s_0_33: u8 = {
            let value = state.read_register::<u8>(s_0_32 as isize);
            tracer.read_register(s_0_32 as isize, value);
            value
        };
        // D s_0_34: write-var ga#11331 <= s_0_33
        fn_state.ga_11331 = s_0_33;
        // D s_0_35: read-var ga#11331:u8
        let s_0_35: u8 = fn_state.ga_11331;
        // D s_0_36: cast zx s_0_35 -> bv
        let s_0_36: Bits = Bits::new(s_0_35 as u128, 2u16);
        // C s_0_37: const #448u : u32
        let s_0_37: u32 = 448;
        // D s_0_38: read-reg s_0_37:u8
        let s_0_38: u8 = {
            let value = state.read_register::<u8>(s_0_37 as isize);
            tracer.read_register(s_0_37 as isize, value);
            value
        };
        // D s_0_39: cast zx s_0_38 -> bv
        let s_0_39: Bits = Bits::new(s_0_38 as u128, 2u16);
        // D s_0_40: cmp-eq s_0_36 s_0_39
        let s_0_40: bool = ((s_0_36) == (s_0_39));
        // D s_0_41: not s_0_40
        let s_0_41: bool = !s_0_40;
        // N s_0_42: branch s_0_41 b30 b1
        if s_0_41 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call EL2Enabled(s_1_0)
        let s_1_1: bool = EL2Enabled(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b29 b2
        if s_1_1 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#15299 <= s_2_0
        fn_state.gs_15299 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var gs#15299:u8
        let s_3_0: bool = fn_state.gs_15299;
        // N s_3_1: branch s_3_0 b25 b4
        if s_3_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#15301 <= s_4_0
        fn_state.gs_15301 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_5_0: read-var gs#15301:u8
        let s_5_0: bool = fn_state.gs_15301;
        // D s_5_1: write-var TrapEL2 <= s_5_0
        fn_state.TrapEL2 = s_5_0;
        // C s_5_2: const #424u : u32
        let s_5_2: u32 = 424;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // C s_5_4: const #2u : u8
        let s_5_4: u8 = 2;
        // D s_5_5: cmp-lt s_5_3 s_5_4
        let s_5_5: bool = ((s_5_3) < (s_5_4));
        // N s_5_6: branch s_5_5 b24 b6
        if s_5_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#15303 <= s_6_0
        fn_state.gs_15303 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var gs#15303:u8
        let s_7_0: bool = fn_state.gs_15303;
        // D s_7_1: write-var TrapEL3 <= s_7_0
        fn_state.TrapEL3 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_8_0: read-var TrapEL3:u8
        let s_8_0: bool = fn_state.TrapEL3;
        // N s_8_1: branch s_8_0 b23 b9
        if s_8_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#15316 <= s_9_0
        fn_state.gs_15316 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_10_0: read-var gs#15316:u8
        let s_10_0: bool = fn_state.gs_15316;
        // N s_10_1: branch s_10_0 b22 b11
        if s_10_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_11_0: read-var TrapEL2:u8
        let s_11_0: bool = fn_state.TrapEL2;
        // N s_11_1: branch s_11_0 b20 b12
        if s_11_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_12_0: read-var TrapEL3:u8
        let s_12_0: bool = fn_state.TrapEL3;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_13_0: const #64s : i
        let s_13_0: i128 = 64;
        // D s_13_1: read-var APGAKey_EL1:u128
        let s_13_1: u128 = fn_state.APGAKey_EL1;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 128u16);
        // C s_13_3: const #1s : i64
        let s_13_3: i64 = 1;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #63s : i
        let s_13_5: i128 = 63;
        // C s_13_6: add s_13_5 s_13_4
        let s_13_6: i128 = (s_13_5 + s_13_4);
        // D s_13_7: bit-extract s_13_2 s_13_0 s_13_6
        let s_13_7: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_6).unwrap(),
        ));
        // D s_13_8: cast reint s_13_7 -> u64
        let s_13_8: u64 = (s_13_7.value() as u64);
        // C s_13_9: const #0s : i
        let s_13_9: i128 = 0;
        // D s_13_10: read-var APGAKey_EL1:u128
        let s_13_10: u128 = fn_state.APGAKey_EL1;
        // D s_13_11: cast zx s_13_10 -> bv
        let s_13_11: Bits = Bits::new(s_13_10 as u128, 128u16);
        // C s_13_12: const #1s : i64
        let s_13_12: i64 = 1;
        // C s_13_13: cast zx s_13_12 -> i
        let s_13_13: i128 = (i128::try_from(s_13_12).unwrap());
        // C s_13_14: const #63s : i
        let s_13_14: i128 = 63;
        // C s_13_15: add s_13_14 s_13_13
        let s_13_15: i128 = (s_13_14 + s_13_13);
        // D s_13_16: bit-extract s_13_11 s_13_9 s_13_15
        let s_13_16: Bits = (Bits::new(
            ((s_13_11) >> (s_13_9)).value(),
            u16::try_from(s_13_15).unwrap(),
        ));
        // D s_13_17: cast reint s_13_16 -> u64
        let s_13_17: u64 = (s_13_16.value() as u64);
        // D s_13_18: read-var x:u64
        let s_13_18: u64 = fn_state.x;
        // D s_13_19: read-var y:u64
        let s_13_19: u64 = fn_state.y;
        // C s_13_20: const #1u : u8
        let s_13_20: bool = true;
        // D s_13_21: call ComputePAC(s_13_18, s_13_19, s_13_8, s_13_17, s_13_20)
        let s_13_21: u64 = ComputePAC(
            state,
            tracer,
            s_13_18,
            s_13_19,
            s_13_8,
            s_13_17,
            s_13_20,
        );
        // C s_13_22: const #32s : i
        let s_13_22: i128 = 32;
        // D s_13_23: cast zx s_13_21 -> bv
        let s_13_23: Bits = Bits::new(s_13_21 as u128, 64u16);
        // C s_13_24: const #1s : i64
        let s_13_24: i64 = 1;
        // C s_13_25: cast zx s_13_24 -> i
        let s_13_25: i128 = (i128::try_from(s_13_24).unwrap());
        // C s_13_26: const #31s : i
        let s_13_26: i128 = 31;
        // C s_13_27: add s_13_26 s_13_25
        let s_13_27: i128 = (s_13_26 + s_13_25);
        // D s_13_28: bit-extract s_13_23 s_13_22 s_13_27
        let s_13_28: Bits = (Bits::new(
            ((s_13_23) >> (s_13_22)).value(),
            u16::try_from(s_13_27).unwrap(),
        ));
        // D s_13_29: cast reint s_13_28 -> u32
        let s_13_29: u32 = (s_13_28.value() as u32);
        // C s_13_30: const #32s : i
        let s_13_30: i128 = 32;
        // S s_13_31: call Zeros(s_13_30)
        let s_13_31: Bits = Zeros(state, tracer, s_13_30);
        // S s_13_32: cast reint s_13_31 -> u32
        let s_13_32: u32 = (s_13_31.value() as u32);
        // D s_13_33: cast zx s_13_29 -> bv
        let s_13_33: Bits = Bits::new(s_13_29 as u128, 32u16);
        // S s_13_34: cast zx s_13_32 -> bv
        let s_13_34: Bits = Bits::new(s_13_32 as u128, 32u16);
        // D s_13_35: cast reint s_13_33 -> u128
        let s_13_35: u128 = (s_13_33.value() as u128);
        // D s_13_36: size-of s_13_33
        let s_13_36: u16 = s_13_33.length();
        // S s_13_37: cast reint s_13_34 -> u128
        let s_13_37: u128 = (s_13_34.value() as u128);
        // D s_13_38: size-of s_13_34
        let s_13_38: u16 = s_13_34.length();
        // D s_13_39: lsl s_13_35 s_13_38
        let s_13_39: u128 = s_13_35 << s_13_38;
        // D s_13_40: or s_13_39 s_13_37
        let s_13_40: u128 = ((s_13_39) | (s_13_37));
        // D s_13_41: add s_13_36 s_13_38
        let s_13_41: u16 = (s_13_36 + s_13_38);
        // D s_13_42: create-bits s_13_40 s_13_41
        let s_13_42: Bits = Bits::new(s_13_40, s_13_41);
        // D s_13_43: cast reint s_13_42 -> u64
        let s_13_43: u64 = (s_13_42.value() as u64);
        // D s_13_44: write-var return_value <= s_13_43
        fn_state.return_value = s_13_43;
        // N s_13_45: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_14_0: read-var return_value:u64
        let s_14_0: u64 = fn_state.return_value;
        // N s_14_1: return s_14_0
        return s_14_0;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call EL3SDDUndef(s_15_0)
        let s_15_1: bool = EL3SDDUndef(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b18 b16
        if s_15_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call TrapPACUse(s_16_1)
        let s_16_2: () = TrapPACUse(state, tracer, s_16_1);
        // N s_16_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var ga#11353:u64
        let s_17_0: u64 = fn_state.ga_11353;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_19_0: read-var return_value:u64
        let s_19_0: u64 = fn_state.return_value;
        // N s_19_1: return s_19_0
        return s_19_0;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_20_0: const #432u : u32
        let s_20_0: u32 = 432;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call TrapPACUse(s_20_1)
        let s_20_2: () = TrapPACUse(state, tracer, s_20_1);
        // N s_20_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_21_0: read-var ga#11349:u64
        let s_21_0: u64 = fn_state.ga_11349;
        // D s_21_1: write-var return_value <= s_21_0
        fn_state.return_value = s_21_0;
        // N s_21_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call EL3SDDUndefPriority(s_23_0)
        let s_23_1: bool = EL3SDDUndefPriority(state, tracer, s_23_0);
        // D s_23_2: write-var gs#15316 <= s_23_1
        fn_state.gs_15316 = s_23_1;
        // N s_23_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_24_0: const #90704u : u32
        let s_24_0: u32 = 90704;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_SCR_EL3_Type_API(s_24_1)
        let s_24_2: bool = u_get_SCR_EL3_Type_API(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #0u : u8
        let s_24_4: bool = false;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // D s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var gs#15303 <= s_24_6
        fn_state.gs_15303 = s_24_6;
        // N s_24_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_25_0: const #102552u : u32
        let s_25_0: u32 = 102552;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_HCR_EL2_Type_TGE(s_25_1)
        let s_25_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #0u : u8
        let s_25_4: bool = false;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // N s_25_7: branch s_25_6 b28 b26
        if s_25_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_26_0: const #102552u : u32
        let s_26_0: u32 = 102552;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_HCR_EL2_Type_E2H(s_26_1)
        let s_26_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_26_1);
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // C s_26_4: const #0u : u8
        let s_26_4: bool = false;
        // C s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 1u16);
        // D s_26_6: cmp-eq s_26_3 s_26_5
        let s_26_6: bool = ((s_26_3) == (s_26_5));
        // D s_26_7: write-var gs#15300 <= s_26_6
        fn_state.gs_15300 = s_26_6;
        // N s_26_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_27_0: read-var gs#15300:u8
        let s_27_0: bool = fn_state.gs_15300;
        // D s_27_1: write-var gs#15301 <= s_27_0
        fn_state.gs_15301 = s_27_0;
        // N s_27_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#15300 <= s_28_0
        fn_state.gs_15300 = s_28_0;
        // N s_28_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_29_0: const #102552u : u32
        let s_29_0: u32 = 102552;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_HCR_EL2_Type_API(s_29_1)
        let s_29_2: bool = u_get_HCR_EL2_Type_API(state, tracer, s_29_1);
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #0u : u8
        let s_29_4: bool = false;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // D s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // D s_29_7: write-var gs#15299 <= s_29_6
        fn_state.gs_15299 = s_29_6;
        // N s_29_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_30_0: read-var ga#11331:u8
        let s_30_0: u8 = fn_state.ga_11331;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 2u16);
        // C s_30_2: const #440u : u32
        let s_30_2: u32 = 440;
        // D s_30_3: read-reg s_30_2:u8
        let s_30_3: u8 = {
            let value = state.read_register::<u8>(s_30_2 as isize);
            tracer.read_register(s_30_2 as isize, value);
            value
        };
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 2u16);
        // D s_30_5: cmp-eq s_30_1 s_30_4
        let s_30_5: bool = ((s_30_1) == (s_30_4));
        // D s_30_6: not s_30_5
        let s_30_6: bool = !s_30_5;
        // N s_30_7: branch s_30_6 b38 b31
        if s_30_6 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call EL2Enabled(s_31_0)
        let s_31_1: bool = EL2Enabled(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b37 b32
        if s_31_1 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#15305 <= s_32_0
        fn_state.gs_15305 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_33_0: read-var gs#15305:u8
        let s_33_0: bool = fn_state.gs_15305;
        // D s_33_1: write-var TrapEL2 <= s_33_0
        fn_state.TrapEL2 = s_33_0;
        // C s_33_2: const #424u : u32
        let s_33_2: u32 = 424;
        // D s_33_3: read-reg s_33_2:u8
        let s_33_3: u8 = {
            let value = state.read_register::<u8>(s_33_2 as isize);
            tracer.read_register(s_33_2 as isize, value);
            value
        };
        // C s_33_4: const #2u : u8
        let s_33_4: u8 = 2;
        // D s_33_5: cmp-lt s_33_3 s_33_4
        let s_33_5: bool = ((s_33_3) < (s_33_4));
        // N s_33_6: branch s_33_5 b36 b34
        if s_33_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#15307 <= s_34_0
        fn_state.gs_15307 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_35_0: read-var gs#15307:u8
        let s_35_0: bool = fn_state.gs_15307;
        // D s_35_1: write-var TrapEL3 <= s_35_0
        fn_state.TrapEL3 = s_35_0;
        // N s_35_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_36_0: const #90704u : u32
        let s_36_0: u32 = 90704;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_SCR_EL3_Type_API(s_36_1)
        let s_36_2: bool = u_get_SCR_EL3_Type_API(state, tracer, s_36_1);
        // D s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #0u : u8
        let s_36_4: bool = false;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // D s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // D s_36_7: write-var gs#15307 <= s_36_6
        fn_state.gs_15307 = s_36_6;
        // N s_36_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_37_0: const #102552u : u32
        let s_37_0: u32 = 102552;
        // D s_37_1: read-reg s_37_0:struct
        let s_37_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call _get_HCR_EL2_Type_API(s_37_1)
        let s_37_2: bool = u_get_HCR_EL2_Type_API(state, tracer, s_37_1);
        // D s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // C s_37_4: const #0u : u8
        let s_37_4: bool = false;
        // C s_37_5: cast zx s_37_4 -> bv
        let s_37_5: Bits = Bits::new(s_37_4 as u128, 1u16);
        // D s_37_6: cmp-eq s_37_3 s_37_5
        let s_37_6: bool = ((s_37_3) == (s_37_5));
        // D s_37_7: write-var gs#15305 <= s_37_6
        fn_state.gs_15305 = s_37_6;
        // N s_37_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_38_0: read-var ga#11331:u8
        let s_38_0: u8 = fn_state.ga_11331;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 2u16);
        // C s_38_2: const #432u : u32
        let s_38_2: u32 = 432;
        // D s_38_3: read-reg s_38_2:u8
        let s_38_3: u8 = {
            let value = state.read_register::<u8>(s_38_2 as isize);
            tracer.read_register(s_38_2 as isize, value);
            value
        };
        // D s_38_4: cast zx s_38_3 -> bv
        let s_38_4: Bits = Bits::new(s_38_3 as u128, 2u16);
        // D s_38_5: cmp-eq s_38_1 s_38_4
        let s_38_5: bool = ((s_38_1) == (s_38_4));
        // D s_38_6: not s_38_5
        let s_38_6: bool = !s_38_5;
        // N s_38_7: branch s_38_6 b43 b39
        if s_38_6 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var TrapEL2 <= s_39_0
        fn_state.TrapEL2 = s_39_0;
        // C s_39_2: const #424u : u32
        let s_39_2: u32 = 424;
        // D s_39_3: read-reg s_39_2:u8
        let s_39_3: u8 = {
            let value = state.read_register::<u8>(s_39_2 as isize);
            tracer.read_register(s_39_2 as isize, value);
            value
        };
        // C s_39_4: const #2u : u8
        let s_39_4: u8 = 2;
        // D s_39_5: cmp-lt s_39_3 s_39_4
        let s_39_5: bool = ((s_39_3) < (s_39_4));
        // N s_39_6: branch s_39_5 b42 b40
        if s_39_5 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#15310 <= s_40_0
        fn_state.gs_15310 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_41_0: read-var gs#15310:u8
        let s_41_0: bool = fn_state.gs_15310;
        // D s_41_1: write-var TrapEL3 <= s_41_0
        fn_state.TrapEL3 = s_41_0;
        // N s_41_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_42_0: const #90704u : u32
        let s_42_0: u32 = 90704;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_SCR_EL3_Type_API(s_42_1)
        let s_42_2: bool = u_get_SCR_EL3_Type_API(state, tracer, s_42_1);
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // C s_42_4: const #0u : u8
        let s_42_4: bool = false;
        // C s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // D s_42_6: cmp-eq s_42_3 s_42_5
        let s_42_6: bool = ((s_42_3) == (s_42_5));
        // D s_42_7: write-var gs#15310 <= s_42_6
        fn_state.gs_15310 = s_42_6;
        // N s_42_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_43_0: read-var ga#11331:u8
        let s_43_0: u8 = fn_state.ga_11331;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 2u16);
        // C s_43_2: const #424u : u32
        let s_43_2: u32 = 424;
        // D s_43_3: read-reg s_43_2:u8
        let s_43_3: u8 = {
            let value = state.read_register::<u8>(s_43_2 as isize);
            tracer.read_register(s_43_2 as isize, value);
            value
        };
        // D s_43_4: cast zx s_43_3 -> bv
        let s_43_4: Bits = Bits::new(s_43_3 as u128, 2u16);
        // D s_43_5: cmp-eq s_43_1 s_43_4
        let s_43_5: bool = ((s_43_1) == (s_43_4));
        // D s_43_6: not s_43_5
        let s_43_6: bool = !s_43_5;
        // N s_43_7: branch s_43_6 b45 b44
        if s_43_6 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var TrapEL2 <= s_44_0
        fn_state.TrapEL2 = s_44_0;
        // C s_44_2: const #0u : u8
        let s_44_2: bool = false;
        // D s_44_3: write-var TrapEL3 <= s_44_2
        fn_state.TrapEL3 = s_44_2;
        // N s_44_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_45_0: jump b8
        return block_8(state, tracer, fn_state);
    }
}
