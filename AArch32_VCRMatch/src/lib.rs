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
use ExcVectorBase::*;
use IsZero::*;
use ELUsingAArch32::*;
use DBGVCR_read::*;
use ConstrainUnpredictableBool::*;
use UsingAArch32::*;
use CurrentSecurityState::*;
use Zeros::*;
use Bit::*;
use DebugTarget::*;
use common::*;
pub fn AArch32_VCRMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_30066: bool,
        gs_30051: bool,
        ga_23288: ProductType700c18a878c5601b,
        mask: u32,
        gs_30086: bool,
        gs_30065: bool,
        val_match: bool,
        gs_30072: bool,
        gs_30052: bool,
        ssshadow_551: u32,
        match_word: u32,
        gs_30060: bool,
        gs_30089: bool,
        vaddress: u32,
    }
    let fn_state = FunctionState {
        vaddress,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b46 b1
        if s_0_1 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#30051 <= s_1_0
        fn_state.gs_30051 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#30051:u8
        let s_2_0: bool = fn_state.gs_30051;
        // N s_2_1: branch s_2_0 b45 b3
        if s_2_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#30052 <= s_3_0
        fn_state.gs_30052 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#30052:u8
        let s_4_0: bool = fn_state.gs_30052;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var val_match <= s_5_0
        fn_state.val_match = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var val_match:u8
        let s_6_0: bool = fn_state.val_match;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #32s : i
        let s_7_0: i128 = 32;
        // S s_7_1: call Zeros(s_7_0)
        let s_7_1: Bits = Zeros(state, tracer, s_7_0);
        // S s_7_2: cast reint s_7_1 -> u32
        let s_7_2: u32 = (s_7_1.value() as u32);
        // D s_7_3: write-var match_word <= s_7_2
        fn_state.match_word = s_7_2;
        // C s_7_4: const #() : ()
        let s_7_4: () = ();
        // S s_7_5: call CurrentSecurityState(s_7_4)
        let s_7_5: u32 = CurrentSecurityState(state, tracer, s_7_4);
        // D s_7_6: write-var ssshadow#551 <= s_7_5
        fn_state.ssshadow_551 = s_7_5;
        // C s_7_7: const #5s : i
        let s_7_7: i128 = 5;
        // D s_7_8: read-var vaddress:u32
        let s_7_8: u32 = fn_state.vaddress;
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 32u16);
        // C s_7_10: const #1s : i64
        let s_7_10: i64 = 1;
        // C s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // C s_7_12: const #26s : i
        let s_7_12: i128 = 26;
        // C s_7_13: add s_7_12 s_7_11
        let s_7_13: i128 = (s_7_12 + s_7_11);
        // D s_7_14: bit-extract s_7_9 s_7_7 s_7_13
        let s_7_14: Bits = (Bits::new(
            ((s_7_9) >> (s_7_7)).value(),
            u16::try_from(s_7_13).unwrap(),
        ));
        // D s_7_15: cast reint s_7_14 -> u27
        let s_7_15: u32 = (s_7_14.value() as u32);
        // C s_7_16: const #() : ()
        let s_7_16: () = ();
        // S s_7_17: call ExcVectorBase(s_7_16)
        let s_7_17: u32 = ExcVectorBase(state, tracer, s_7_16);
        // C s_7_18: const #5s : i
        let s_7_18: i128 = 5;
        // S s_7_19: cast zx s_7_17 -> bv
        let s_7_19: Bits = Bits::new(s_7_17 as u128, 32u16);
        // C s_7_20: const #1s : i64
        let s_7_20: i64 = 1;
        // C s_7_21: cast zx s_7_20 -> i
        let s_7_21: i128 = (i128::try_from(s_7_20).unwrap());
        // C s_7_22: const #26s : i
        let s_7_22: i128 = 26;
        // C s_7_23: add s_7_22 s_7_21
        let s_7_23: i128 = (s_7_22 + s_7_21);
        // D s_7_24: bit-extract s_7_19 s_7_18 s_7_23
        let s_7_24: Bits = (Bits::new(
            ((s_7_19) >> (s_7_18)).value(),
            u16::try_from(s_7_23).unwrap(),
        ));
        // D s_7_25: cast reint s_7_24 -> u27
        let s_7_25: u32 = (s_7_24.value() as u32);
        // D s_7_26: cast zx s_7_15 -> bv
        let s_7_26: Bits = Bits::new(s_7_15 as u128, 27u16);
        // D s_7_27: cast zx s_7_25 -> bv
        let s_7_27: Bits = Bits::new(s_7_25 as u128, 27u16);
        // D s_7_28: cmp-eq s_7_26 s_7_27
        let s_7_28: bool = ((s_7_26) == (s_7_27));
        // N s_7_29: branch s_7_28 b39 b8
        if s_7_28 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #424u : u32
        let s_9_0: u32 = 424;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // C s_9_2: const #2u : u8
        let s_9_2: u8 = 2;
        // D s_9_3: cmp-lt s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) < (s_9_2));
        // N s_9_4: branch s_9_3 b38 b10
        if s_9_3 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#30060 <= s_10_0
        fn_state.gs_30060 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#30060:u8
        let s_11_0: bool = fn_state.gs_30060;
        // N s_11_1: branch s_11_0 b37 b12
        if s_11_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#30065 <= s_12_0
        fn_state.gs_30065 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#30065:u8
        let s_13_0: bool = fn_state.gs_30065;
        // N s_13_1: branch s_13_0 b36 b14
        if s_13_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#30066 <= s_14_0
        fn_state.gs_30066 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var gs#30066:u8
        let s_15_0: bool = fn_state.gs_30066;
        // N s_15_1: branch s_15_0 b35 b16
        if s_15_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #424u : u32
        let s_17_0: u32 = 424;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // D s_17_4: not s_17_3
        let s_17_4: bool = !s_17_3;
        // N s_17_5: branch s_17_4 b34 b18
        if s_17_4 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #424u : u32
        let s_18_0: u32 = 424;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call ELUsingAArch32(s_18_1)
        let s_18_2: bool = ELUsingAArch32(state, tracer, s_18_1);
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b33 b19
        if s_18_3 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #222u : u8
        let s_19_0: u8 = 222;
        // C s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 8u16);
        // C s_19_2: const #0u : u8
        let s_19_2: u8 = 0;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 8u16);
        // C s_19_4: cast reint s_19_1 -> u128
        let s_19_4: u128 = (s_19_1.value() as u128);
        // D s_19_5: size-of s_19_1
        let s_19_5: u16 = s_19_1.length();
        // C s_19_6: cast reint s_19_3 -> u128
        let s_19_6: u128 = (s_19_3.value() as u128);
        // D s_19_7: size-of s_19_3
        let s_19_7: u16 = s_19_3.length();
        // D s_19_8: lsl s_19_4 s_19_7
        let s_19_8: u128 = s_19_4 << s_19_7;
        // D s_19_9: or s_19_8 s_19_6
        let s_19_9: u128 = ((s_19_8) | (s_19_6));
        // D s_19_10: add s_19_5 s_19_7
        let s_19_10: u16 = (s_19_5 + s_19_7);
        // D s_19_11: create-bits s_19_9 s_19_10
        let s_19_11: Bits = Bits::new(s_19_9, s_19_10);
        // D s_19_12: cast reint s_19_11 -> u16
        let s_19_12: u16 = (s_19_11.value() as u16);
        // D s_19_13: cast zx s_19_12 -> bv
        let s_19_13: Bits = Bits::new(s_19_12 as u128, 16u16);
        // C s_19_14: const #220u : u8
        let s_19_14: u8 = 220;
        // C s_19_15: cast zx s_19_14 -> bv
        let s_19_15: Bits = Bits::new(s_19_14 as u128, 8u16);
        // D s_19_16: cast reint s_19_13 -> u128
        let s_19_16: u128 = (s_19_13.value() as u128);
        // D s_19_17: size-of s_19_13
        let s_19_17: u16 = s_19_13.length();
        // C s_19_18: cast reint s_19_15 -> u128
        let s_19_18: u128 = (s_19_15.value() as u128);
        // D s_19_19: size-of s_19_15
        let s_19_19: u16 = s_19_15.length();
        // D s_19_20: lsl s_19_16 s_19_19
        let s_19_20: u128 = s_19_16 << s_19_19;
        // D s_19_21: or s_19_20 s_19_18
        let s_19_21: u128 = ((s_19_20) | (s_19_18));
        // D s_19_22: add s_19_17 s_19_19
        let s_19_22: u16 = (s_19_17 + s_19_19);
        // D s_19_23: create-bits s_19_21 s_19_22
        let s_19_23: Bits = Bits::new(s_19_21, s_19_22);
        // D s_19_24: cast reint s_19_23 -> u24
        let s_19_24: u32 = (s_19_23.value() as u32);
        // D s_19_25: cast zx s_19_24 -> bv
        let s_19_25: Bits = Bits::new(s_19_24 as u128, 24u16);
        // C s_19_26: const #222u : u8
        let s_19_26: u8 = 222;
        // C s_19_27: cast zx s_19_26 -> bv
        let s_19_27: Bits = Bits::new(s_19_26 as u128, 8u16);
        // D s_19_28: cast reint s_19_25 -> u128
        let s_19_28: u128 = (s_19_25.value() as u128);
        // D s_19_29: size-of s_19_25
        let s_19_29: u16 = s_19_25.length();
        // C s_19_30: cast reint s_19_27 -> u128
        let s_19_30: u128 = (s_19_27.value() as u128);
        // D s_19_31: size-of s_19_27
        let s_19_31: u16 = s_19_27.length();
        // D s_19_32: lsl s_19_28 s_19_31
        let s_19_32: u128 = s_19_28 << s_19_31;
        // D s_19_33: or s_19_32 s_19_30
        let s_19_33: u128 = ((s_19_32) | (s_19_30));
        // D s_19_34: add s_19_29 s_19_31
        let s_19_34: u16 = (s_19_29 + s_19_31);
        // D s_19_35: create-bits s_19_33 s_19_34
        let s_19_35: Bits = Bits::new(s_19_33, s_19_34);
        // D s_19_36: cast reint s_19_35 -> u32
        let s_19_36: u32 = (s_19_35.value() as u32);
        // D s_19_37: write-var mask <= s_19_36
        fn_state.mask = s_19_36;
        // N s_19_38: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call DBGVCR_read(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = DBGVCR_read(state, tracer, s_20_0);
        // D s_20_2: write-var ga#23288 <= s_20_1
        fn_state.ga_23288 = s_20_1;
        // D s_20_3: read-var ga#23288.0:struct
        let s_20_3: u32 = fn_state.ga_23288._0;
        // D s_20_4: read-var match_word:u32
        let s_20_4: u32 = fn_state.match_word;
        // D s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 32u16);
        // D s_20_6: cast zx s_20_3 -> bv
        let s_20_6: Bits = Bits::new(s_20_3 as u128, 32u16);
        // D s_20_7: and s_20_5 s_20_6
        let s_20_7: Bits = ((s_20_5) & (s_20_6));
        // D s_20_8: cast reint s_20_7 -> u32
        let s_20_8: u32 = (s_20_7.value() as u32);
        // D s_20_9: cast zx s_20_8 -> bv
        let s_20_9: Bits = Bits::new(s_20_8 as u128, 32u16);
        // D s_20_10: read-var mask:u32
        let s_20_10: u32 = fn_state.mask;
        // D s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 32u16);
        // D s_20_12: and s_20_9 s_20_11
        let s_20_12: Bits = ((s_20_9) & (s_20_11));
        // D s_20_13: cast reint s_20_12 -> u32
        let s_20_13: u32 = (s_20_12.value() as u32);
        // D s_20_14: cast zx s_20_13 -> bv
        let s_20_14: Bits = Bits::new(s_20_13 as u128, 32u16);
        // D s_20_15: call IsZero(s_20_14)
        let s_20_15: bool = IsZero(state, tracer, s_20_14);
        // D s_20_16: not s_20_15
        let s_20_16: bool = !s_20_15;
        // D s_20_17: write-var val_match <= s_20_16
        fn_state.val_match = s_20_16;
        // C s_20_18: const #27s : i
        let s_20_18: i128 = 27;
        // D s_20_19: cast zx s_20_13 -> bv
        let s_20_19: Bits = Bits::new(s_20_13 as u128, 32u16);
        // C s_20_20: const #1s : i64
        let s_20_20: i64 = 1;
        // C s_20_21: cast zx s_20_20 -> i
        let s_20_21: i128 = (i128::try_from(s_20_20).unwrap());
        // C s_20_22: const #1s : i
        let s_20_22: i128 = 1;
        // C s_20_23: add s_20_22 s_20_21
        let s_20_23: i128 = (s_20_22 + s_20_21);
        // D s_20_24: bit-extract s_20_19 s_20_18 s_20_23
        let s_20_24: Bits = (Bits::new(
            ((s_20_19) >> (s_20_18)).value(),
            u16::try_from(s_20_23).unwrap(),
        ));
        // D s_20_25: cast reint s_20_24 -> u8
        let s_20_25: u8 = (s_20_24.value() as u8);
        // C s_20_26: const #11s : i
        let s_20_26: i128 = 11;
        // D s_20_27: cast zx s_20_13 -> bv
        let s_20_27: Bits = Bits::new(s_20_13 as u128, 32u16);
        // C s_20_28: const #1s : i64
        let s_20_28: i64 = 1;
        // C s_20_29: cast zx s_20_28 -> i
        let s_20_29: i128 = (i128::try_from(s_20_28).unwrap());
        // C s_20_30: const #1s : i
        let s_20_30: i128 = 1;
        // C s_20_31: add s_20_30 s_20_29
        let s_20_31: i128 = (s_20_30 + s_20_29);
        // D s_20_32: bit-extract s_20_27 s_20_26 s_20_31
        let s_20_32: Bits = (Bits::new(
            ((s_20_27) >> (s_20_26)).value(),
            u16::try_from(s_20_31).unwrap(),
        ));
        // D s_20_33: cast reint s_20_32 -> u8
        let s_20_33: u8 = (s_20_32.value() as u8);
        // C s_20_34: const #3s : i
        let s_20_34: i128 = 3;
        // D s_20_35: cast zx s_20_13 -> bv
        let s_20_35: Bits = Bits::new(s_20_13 as u128, 32u16);
        // C s_20_36: const #1s : i64
        let s_20_36: i64 = 1;
        // C s_20_37: cast zx s_20_36 -> i
        let s_20_37: i128 = (i128::try_from(s_20_36).unwrap());
        // C s_20_38: const #1s : i
        let s_20_38: i128 = 1;
        // C s_20_39: add s_20_38 s_20_37
        let s_20_39: i128 = (s_20_38 + s_20_37);
        // D s_20_40: bit-extract s_20_35 s_20_34 s_20_39
        let s_20_40: Bits = (Bits::new(
            ((s_20_35) >> (s_20_34)).value(),
            u16::try_from(s_20_39).unwrap(),
        ));
        // D s_20_41: cast reint s_20_40 -> u8
        let s_20_41: u8 = (s_20_40.value() as u8);
        // D s_20_42: cast zx s_20_33 -> bv
        let s_20_42: Bits = Bits::new(s_20_33 as u128, 2u16);
        // D s_20_43: cast zx s_20_41 -> bv
        let s_20_43: Bits = Bits::new(s_20_41 as u128, 2u16);
        // D s_20_44: cast reint s_20_42 -> u128
        let s_20_44: u128 = (s_20_42.value() as u128);
        // D s_20_45: size-of s_20_42
        let s_20_45: u16 = s_20_42.length();
        // D s_20_46: cast reint s_20_43 -> u128
        let s_20_46: u128 = (s_20_43.value() as u128);
        // D s_20_47: size-of s_20_43
        let s_20_47: u16 = s_20_43.length();
        // D s_20_48: lsl s_20_44 s_20_47
        let s_20_48: u128 = s_20_44 << s_20_47;
        // D s_20_49: or s_20_48 s_20_46
        let s_20_49: u128 = ((s_20_48) | (s_20_46));
        // D s_20_50: add s_20_45 s_20_47
        let s_20_50: u16 = (s_20_45 + s_20_47);
        // D s_20_51: create-bits s_20_49 s_20_50
        let s_20_51: Bits = Bits::new(s_20_49, s_20_50);
        // D s_20_52: cast reint s_20_51 -> u8
        let s_20_52: u8 = (s_20_51.value() as u8);
        // D s_20_53: cast zx s_20_25 -> bv
        let s_20_53: Bits = Bits::new(s_20_25 as u128, 2u16);
        // D s_20_54: cast zx s_20_52 -> bv
        let s_20_54: Bits = Bits::new(s_20_52 as u128, 4u16);
        // D s_20_55: cast reint s_20_53 -> u128
        let s_20_55: u128 = (s_20_53.value() as u128);
        // D s_20_56: size-of s_20_53
        let s_20_56: u16 = s_20_53.length();
        // D s_20_57: cast reint s_20_54 -> u128
        let s_20_57: u128 = (s_20_54.value() as u128);
        // D s_20_58: size-of s_20_54
        let s_20_58: u16 = s_20_54.length();
        // D s_20_59: lsl s_20_55 s_20_58
        let s_20_59: u128 = s_20_55 << s_20_58;
        // D s_20_60: or s_20_59 s_20_57
        let s_20_60: u128 = ((s_20_59) | (s_20_57));
        // D s_20_61: add s_20_56 s_20_58
        let s_20_61: u16 = (s_20_56 + s_20_58);
        // D s_20_62: create-bits s_20_60 s_20_61
        let s_20_62: Bits = Bits::new(s_20_60, s_20_61);
        // D s_20_63: cast reint s_20_62 -> u8
        let s_20_63: u8 = (s_20_62.value() as u8);
        // D s_20_64: cast zx s_20_63 -> bv
        let s_20_64: Bits = Bits::new(s_20_63 as u128, 6u16);
        // D s_20_65: call IsZero(s_20_64)
        let s_20_65: bool = IsZero(state, tracer, s_20_64);
        // D s_20_66: not s_20_65
        let s_20_66: bool = !s_20_65;
        // N s_20_67: branch s_20_66 b32 b21
        if s_20_66 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#30086 <= s_21_0
        fn_state.gs_30086 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var gs#30086:u8
        let s_22_0: bool = fn_state.gs_30086;
        // N s_22_1: branch s_22_0 b31 b23
        if s_22_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #0s : i
        let s_24_0: i128 = 0;
        // D s_24_1: read-var vaddress:u32
        let s_24_1: u32 = fn_state.vaddress;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 32u16);
        // C s_24_3: const #1s : i64
        let s_24_3: i64 = 1;
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #1s : i
        let s_24_5: i128 = 1;
        // C s_24_6: add s_24_5 s_24_4
        let s_24_6: i128 = (s_24_5 + s_24_4);
        // D s_24_7: bit-extract s_24_2 s_24_0 s_24_6
        let s_24_7: Bits = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_6).unwrap(),
        ));
        // D s_24_8: cast reint s_24_7 -> u8
        let s_24_8: u8 = (s_24_7.value() as u8);
        // D s_24_9: cast zx s_24_8 -> bv
        let s_24_9: Bits = Bits::new(s_24_8 as u128, 2u16);
        // D s_24_10: call IsZero(s_24_9)
        let s_24_10: bool = IsZero(state, tracer, s_24_9);
        // D s_24_11: not s_24_10
        let s_24_11: bool = !s_24_10;
        // N s_24_12: branch s_24_11 b30 b25
        if s_24_11 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#30089 <= s_25_0
        fn_state.gs_30089 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var gs#30089:u8
        let s_26_0: bool = fn_state.gs_30089;
        // N s_26_1: branch s_26_0 b29 b27
        if s_26_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_28_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #25u : u32
        let s_29_0: u32 = 25;
        // S s_29_1: call ConstrainUnpredictableBool(s_29_0)
        let s_29_1: bool = ConstrainUnpredictableBool(state, tracer, s_29_0);
        // D s_29_2: write-var val_match <= s_29_1
        fn_state.val_match = s_29_1;
        // N s_29_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var val_match:u8
        let s_30_0: bool = fn_state.val_match;
        // D s_30_1: write-var gs#30089 <= s_30_0
        fn_state.gs_30089 = s_30_0;
        // N s_30_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #26u : u32
        let s_31_0: u32 = 26;
        // S s_31_1: call ConstrainUnpredictableBool(s_31_0)
        let s_31_1: bool = ConstrainUnpredictableBool(state, tracer, s_31_0);
        // D s_31_2: write-var val_match <= s_31_1
        fn_state.val_match = s_31_1;
        // N s_31_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call DebugTarget(s_32_0)
        let s_32_1: u8 = DebugTarget(state, tracer, s_32_0);
        // C s_32_2: const #16975u : u32
        let s_32_2: u32 = 16975;
        // D s_32_3: read-reg s_32_2:u8
        let s_32_3: u8 = {
            let value = state.read_register::<u8>(s_32_2 as isize);
            tracer.read_register(s_32_2 as isize, value);
            value
        };
        // S s_32_4: cast zx s_32_1 -> bv
        let s_32_4: Bits = Bits::new(s_32_1 as u128, 2u16);
        // D s_32_5: cast zx s_32_3 -> bv
        let s_32_5: Bits = Bits::new(s_32_3 as u128, 2u16);
        // D s_32_6: cmp-eq s_32_4 s_32_5
        let s_32_6: bool = ((s_32_4) == (s_32_5));
        // D s_32_7: write-var gs#30086 <= s_32_6
        fn_state.gs_30086 = s_32_6;
        // N s_32_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_33_0: const #222u : u8
        let s_33_0: u8 = 222;
        // C s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 8u16);
        // C s_33_2: const #0u : u8
        let s_33_2: u8 = 0;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 8u16);
        // C s_33_4: cast reint s_33_1 -> u128
        let s_33_4: u128 = (s_33_1.value() as u128);
        // D s_33_5: size-of s_33_1
        let s_33_5: u16 = s_33_1.length();
        // C s_33_6: cast reint s_33_3 -> u128
        let s_33_6: u128 = (s_33_3.value() as u128);
        // D s_33_7: size-of s_33_3
        let s_33_7: u16 = s_33_3.length();
        // D s_33_8: lsl s_33_4 s_33_7
        let s_33_8: u128 = s_33_4 << s_33_7;
        // D s_33_9: or s_33_8 s_33_6
        let s_33_9: u128 = ((s_33_8) | (s_33_6));
        // D s_33_10: add s_33_5 s_33_7
        let s_33_10: u16 = (s_33_5 + s_33_7);
        // D s_33_11: create-bits s_33_9 s_33_10
        let s_33_11: Bits = Bits::new(s_33_9, s_33_10);
        // D s_33_12: cast reint s_33_11 -> u16
        let s_33_12: u16 = (s_33_11.value() as u16);
        // D s_33_13: cast zx s_33_12 -> bv
        let s_33_13: Bits = Bits::new(s_33_12 as u128, 16u16);
        // C s_33_14: const #0u : u8
        let s_33_14: u8 = 0;
        // C s_33_15: cast zx s_33_14 -> bv
        let s_33_15: Bits = Bits::new(s_33_14 as u128, 8u16);
        // D s_33_16: cast reint s_33_13 -> u128
        let s_33_16: u128 = (s_33_13.value() as u128);
        // D s_33_17: size-of s_33_13
        let s_33_17: u16 = s_33_13.length();
        // C s_33_18: cast reint s_33_15 -> u128
        let s_33_18: u128 = (s_33_15.value() as u128);
        // D s_33_19: size-of s_33_15
        let s_33_19: u16 = s_33_15.length();
        // D s_33_20: lsl s_33_16 s_33_19
        let s_33_20: u128 = s_33_16 << s_33_19;
        // D s_33_21: or s_33_20 s_33_18
        let s_33_21: u128 = ((s_33_20) | (s_33_18));
        // D s_33_22: add s_33_17 s_33_19
        let s_33_22: u16 = (s_33_17 + s_33_19);
        // D s_33_23: create-bits s_33_21 s_33_22
        let s_33_23: Bits = Bits::new(s_33_21, s_33_22);
        // D s_33_24: cast reint s_33_23 -> u24
        let s_33_24: u32 = (s_33_23.value() as u32);
        // D s_33_25: cast zx s_33_24 -> bv
        let s_33_25: Bits = Bits::new(s_33_24 as u128, 24u16);
        // C s_33_26: const #222u : u8
        let s_33_26: u8 = 222;
        // C s_33_27: cast zx s_33_26 -> bv
        let s_33_27: Bits = Bits::new(s_33_26 as u128, 8u16);
        // D s_33_28: cast reint s_33_25 -> u128
        let s_33_28: u128 = (s_33_25.value() as u128);
        // D s_33_29: size-of s_33_25
        let s_33_29: u16 = s_33_25.length();
        // C s_33_30: cast reint s_33_27 -> u128
        let s_33_30: u128 = (s_33_27.value() as u128);
        // D s_33_31: size-of s_33_27
        let s_33_31: u16 = s_33_27.length();
        // D s_33_32: lsl s_33_28 s_33_31
        let s_33_32: u128 = s_33_28 << s_33_31;
        // D s_33_33: or s_33_32 s_33_30
        let s_33_33: u128 = ((s_33_32) | (s_33_30));
        // D s_33_34: add s_33_29 s_33_31
        let s_33_34: u16 = (s_33_29 + s_33_31);
        // D s_33_35: create-bits s_33_33 s_33_34
        let s_33_35: Bits = Bits::new(s_33_33, s_33_34);
        // D s_33_36: cast reint s_33_35 -> u32
        let s_33_36: u32 = (s_33_35.value() as u32);
        // D s_33_37: write-var mask <= s_33_36
        fn_state.mask = s_33_36;
        // N s_33_38: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #0u : u8
        let s_34_0: u8 = 0;
        // C s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 8u16);
        // C s_34_2: const #0u : u8
        let s_34_2: u8 = 0;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 8u16);
        // C s_34_4: cast reint s_34_1 -> u128
        let s_34_4: u128 = (s_34_1.value() as u128);
        // D s_34_5: size-of s_34_1
        let s_34_5: u16 = s_34_1.length();
        // C s_34_6: cast reint s_34_3 -> u128
        let s_34_6: u128 = (s_34_3.value() as u128);
        // D s_34_7: size-of s_34_3
        let s_34_7: u16 = s_34_3.length();
        // D s_34_8: lsl s_34_4 s_34_7
        let s_34_8: u128 = s_34_4 << s_34_7;
        // D s_34_9: or s_34_8 s_34_6
        let s_34_9: u128 = ((s_34_8) | (s_34_6));
        // D s_34_10: add s_34_5 s_34_7
        let s_34_10: u16 = (s_34_5 + s_34_7);
        // D s_34_11: create-bits s_34_9 s_34_10
        let s_34_11: Bits = Bits::new(s_34_9, s_34_10);
        // D s_34_12: cast reint s_34_11 -> u16
        let s_34_12: u16 = (s_34_11.value() as u16);
        // D s_34_13: cast zx s_34_12 -> bv
        let s_34_13: Bits = Bits::new(s_34_12 as u128, 16u16);
        // C s_34_14: const #0u : u8
        let s_34_14: u8 = 0;
        // C s_34_15: cast zx s_34_14 -> bv
        let s_34_15: Bits = Bits::new(s_34_14 as u128, 8u16);
        // D s_34_16: cast reint s_34_13 -> u128
        let s_34_16: u128 = (s_34_13.value() as u128);
        // D s_34_17: size-of s_34_13
        let s_34_17: u16 = s_34_13.length();
        // C s_34_18: cast reint s_34_15 -> u128
        let s_34_18: u128 = (s_34_15.value() as u128);
        // D s_34_19: size-of s_34_15
        let s_34_19: u16 = s_34_15.length();
        // D s_34_20: lsl s_34_16 s_34_19
        let s_34_20: u128 = s_34_16 << s_34_19;
        // D s_34_21: or s_34_20 s_34_18
        let s_34_21: u128 = ((s_34_20) | (s_34_18));
        // D s_34_22: add s_34_17 s_34_19
        let s_34_22: u16 = (s_34_17 + s_34_19);
        // D s_34_23: create-bits s_34_21 s_34_22
        let s_34_23: Bits = Bits::new(s_34_21, s_34_22);
        // D s_34_24: cast reint s_34_23 -> u24
        let s_34_24: u32 = (s_34_23.value() as u32);
        // D s_34_25: cast zx s_34_24 -> bv
        let s_34_25: Bits = Bits::new(s_34_24 as u128, 24u16);
        // C s_34_26: const #222u : u8
        let s_34_26: u8 = 222;
        // C s_34_27: cast zx s_34_26 -> bv
        let s_34_27: Bits = Bits::new(s_34_26 as u128, 8u16);
        // D s_34_28: cast reint s_34_25 -> u128
        let s_34_28: u128 = (s_34_25.value() as u128);
        // D s_34_29: size-of s_34_25
        let s_34_29: u16 = s_34_25.length();
        // C s_34_30: cast reint s_34_27 -> u128
        let s_34_30: u128 = (s_34_27.value() as u128);
        // D s_34_31: size-of s_34_27
        let s_34_31: u16 = s_34_27.length();
        // D s_34_32: lsl s_34_28 s_34_31
        let s_34_32: u128 = s_34_28 << s_34_31;
        // D s_34_33: or s_34_32 s_34_30
        let s_34_33: u128 = ((s_34_32) | (s_34_30));
        // D s_34_34: add s_34_29 s_34_31
        let s_34_34: u16 = (s_34_29 + s_34_31);
        // D s_34_35: create-bits s_34_33 s_34_34
        let s_34_35: Bits = Bits::new(s_34_33, s_34_34);
        // D s_34_36: cast reint s_34_35 -> u32
        let s_34_36: u32 = (s_34_35.value() as u32);
        // D s_34_37: write-var mask <= s_34_36
        fn_state.mask = s_34_36;
        // N s_34_38: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_35_0: const #2s : i
        let s_35_0: i128 = 2;
        // D s_35_1: read-var vaddress:u32
        let s_35_1: u32 = fn_state.vaddress;
        // D s_35_2: cast zx s_35_1 -> bv
        let s_35_2: Bits = Bits::new(s_35_1 as u128, 32u16);
        // C s_35_3: const #1s : i64
        let s_35_3: i64 = 1;
        // C s_35_4: cast zx s_35_3 -> i
        let s_35_4: i128 = (i128::try_from(s_35_3).unwrap());
        // C s_35_5: const #2s : i
        let s_35_5: i128 = 2;
        // C s_35_6: add s_35_5 s_35_4
        let s_35_6: i128 = (s_35_5 + s_35_4);
        // D s_35_7: bit-extract s_35_2 s_35_0 s_35_6
        let s_35_7: Bits = (Bits::new(
            ((s_35_2) >> (s_35_0)).value(),
            u16::try_from(s_35_6).unwrap(),
        ));
        // D s_35_8: cast reint s_35_7 -> u8
        let s_35_8: u8 = (s_35_7.value() as u8);
        // D s_35_9: cast zx s_35_8 -> bv
        let s_35_9: Bits = Bits::new(s_35_8 as u128, 3u16);
        // D s_35_10: cast zx s_35_9 -> i
        let s_35_10: i128 = (s_35_9.value() as i128);
        // D s_35_11: cast reint s_35_10 -> i64
        let s_35_11: i64 = (s_35_10 as i64);
        // C s_35_12: const #8s : i
        let s_35_12: i128 = 8;
        // D s_35_13: cast zx s_35_11 -> i
        let s_35_13: i128 = (i128::try_from(s_35_11).unwrap());
        // D s_35_14: add s_35_13 s_35_12
        let s_35_14: i128 = (s_35_13 + s_35_12);
        // D s_35_15: cast reint s_35_14 -> i64
        let s_35_15: i64 = (s_35_14 as i64);
        // C s_35_16: const #1u : u8
        let s_35_16: bool = true;
        // S s_35_17: call Bit(s_35_16)
        let s_35_17: bool = Bit(state, tracer, s_35_16);
        // D s_35_18: read-var match_word:u32
        let s_35_18: u32 = fn_state.match_word;
        // D s_35_19: cast zx s_35_18 -> bv
        let s_35_19: Bits = Bits::new(s_35_18 as u128, 32u16);
        // D s_35_20: cast zx s_35_15 -> i
        let s_35_20: i128 = (i128::try_from(s_35_15).unwrap());
        // C s_35_21: const #1u : u64
        let s_35_21: u64 = 1;
        // D s_35_22: bit-insert s_35_19 s_35_19 s_35_20 s_35_21
        let s_35_22: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_35_21 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_35_19.length(),
            );
            (s_35_19 & mask) | (s_35_19 << s_35_20)
        };
        // D s_35_23: cast reint s_35_22 -> u32
        let s_35_23: u32 = (s_35_22.value() as u32);
        // D s_35_24: write-var match_word <= s_35_23
        fn_state.match_word = s_35_23;
        // N s_35_25: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var ssshadow#551:u32
        let s_36_0: u32 = fn_state.ssshadow_551;
        // C s_36_1: const #3u : u32
        let s_36_1: u32 = 3;
        // D s_36_2: cmp-eq s_36_0 s_36_1
        let s_36_2: bool = ((s_36_0) == (s_36_1));
        // D s_36_3: write-var gs#30066 <= s_36_2
        fn_state.gs_30066 = s_36_2;
        // N s_36_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #5s : i
        let s_37_0: i128 = 5;
        // D s_37_1: read-var vaddress:u32
        let s_37_1: u32 = fn_state.vaddress;
        // D s_37_2: cast zx s_37_1 -> bv
        let s_37_2: Bits = Bits::new(s_37_1 as u128, 32u16);
        // C s_37_3: const #1s : i64
        let s_37_3: i64 = 1;
        // C s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // C s_37_5: const #26s : i
        let s_37_5: i128 = 26;
        // C s_37_6: add s_37_5 s_37_4
        let s_37_6: i128 = (s_37_5 + s_37_4);
        // D s_37_7: bit-extract s_37_2 s_37_0 s_37_6
        let s_37_7: Bits = (Bits::new(
            ((s_37_2) >> (s_37_0)).value(),
            u16::try_from(s_37_6).unwrap(),
        ));
        // D s_37_8: cast reint s_37_7 -> u27
        let s_37_8: u32 = (s_37_7.value() as u32);
        // C s_37_9: const #100208u : u32
        let s_37_9: u32 = 100208;
        // D s_37_10: read-reg s_37_9:u32
        let s_37_10: u32 = {
            let value = state.read_register::<u32>(s_37_9 as isize);
            tracer.read_register(s_37_9 as isize, value);
            value
        };
        // C s_37_11: const #5s : i
        let s_37_11: i128 = 5;
        // D s_37_12: cast zx s_37_10 -> bv
        let s_37_12: Bits = Bits::new(s_37_10 as u128, 32u16);
        // C s_37_13: const #1s : i64
        let s_37_13: i64 = 1;
        // C s_37_14: cast zx s_37_13 -> i
        let s_37_14: i128 = (i128::try_from(s_37_13).unwrap());
        // C s_37_15: const #26s : i
        let s_37_15: i128 = 26;
        // C s_37_16: add s_37_15 s_37_14
        let s_37_16: i128 = (s_37_15 + s_37_14);
        // D s_37_17: bit-extract s_37_12 s_37_11 s_37_16
        let s_37_17: Bits = (Bits::new(
            ((s_37_12) >> (s_37_11)).value(),
            u16::try_from(s_37_16).unwrap(),
        ));
        // D s_37_18: cast reint s_37_17 -> u27
        let s_37_18: u32 = (s_37_17.value() as u32);
        // D s_37_19: cast zx s_37_8 -> bv
        let s_37_19: Bits = Bits::new(s_37_8 as u128, 27u16);
        // D s_37_20: cast zx s_37_18 -> bv
        let s_37_20: Bits = Bits::new(s_37_18 as u128, 27u16);
        // D s_37_21: cmp-eq s_37_19 s_37_20
        let s_37_21: bool = ((s_37_19) == (s_37_20));
        // D s_37_22: write-var gs#30065 <= s_37_21
        fn_state.gs_30065 = s_37_21;
        // N s_37_23: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #424u : u32
        let s_38_0: u32 = 424;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call ELUsingAArch32(s_38_1)
        let s_38_2: bool = ELUsingAArch32(state, tracer, s_38_1);
        // D s_38_3: write-var gs#30060 <= s_38_2
        fn_state.gs_30060 = s_38_2;
        // N s_38_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #424u : u32
        let s_39_0: u32 = 424;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: u8 = {
            let value = state.read_register::<u8>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // C s_39_2: const #2u : u8
        let s_39_2: u8 = 2;
        // D s_39_3: cmp-lt s_39_1 s_39_2
        let s_39_3: bool = ((s_39_1) < (s_39_2));
        // N s_39_4: branch s_39_3 b44 b40
        if s_39_3 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#30072 <= s_40_0
        fn_state.gs_30072 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_41_0: read-var gs#30072:u8
        let s_41_0: bool = fn_state.gs_30072;
        // N s_41_1: branch s_41_0 b43 b42
        if s_41_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_42_0: const #2s : i
        let s_42_0: i128 = 2;
        // D s_42_1: read-var vaddress:u32
        let s_42_1: u32 = fn_state.vaddress;
        // D s_42_2: cast zx s_42_1 -> bv
        let s_42_2: Bits = Bits::new(s_42_1 as u128, 32u16);
        // C s_42_3: const #1s : i64
        let s_42_3: i64 = 1;
        // C s_42_4: cast zx s_42_3 -> i
        let s_42_4: i128 = (i128::try_from(s_42_3).unwrap());
        // C s_42_5: const #2s : i
        let s_42_5: i128 = 2;
        // C s_42_6: add s_42_5 s_42_4
        let s_42_6: i128 = (s_42_5 + s_42_4);
        // D s_42_7: bit-extract s_42_2 s_42_0 s_42_6
        let s_42_7: Bits = (Bits::new(
            ((s_42_2) >> (s_42_0)).value(),
            u16::try_from(s_42_6).unwrap(),
        ));
        // D s_42_8: cast reint s_42_7 -> u8
        let s_42_8: u8 = (s_42_7.value() as u8);
        // D s_42_9: cast zx s_42_8 -> bv
        let s_42_9: Bits = Bits::new(s_42_8 as u128, 3u16);
        // D s_42_10: cast zx s_42_9 -> i
        let s_42_10: i128 = (s_42_9.value() as i128);
        // D s_42_11: cast reint s_42_10 -> i64
        let s_42_11: i64 = (s_42_10 as i64);
        // C s_42_12: const #0s : i
        let s_42_12: i128 = 0;
        // D s_42_13: cast zx s_42_11 -> i
        let s_42_13: i128 = (i128::try_from(s_42_11).unwrap());
        // D s_42_14: add s_42_13 s_42_12
        let s_42_14: i128 = (s_42_13 + s_42_12);
        // D s_42_15: cast reint s_42_14 -> i64
        let s_42_15: i64 = (s_42_14 as i64);
        // C s_42_16: const #1u : u8
        let s_42_16: bool = true;
        // S s_42_17: call Bit(s_42_16)
        let s_42_17: bool = Bit(state, tracer, s_42_16);
        // D s_42_18: read-var match_word:u32
        let s_42_18: u32 = fn_state.match_word;
        // D s_42_19: cast zx s_42_18 -> bv
        let s_42_19: Bits = Bits::new(s_42_18 as u128, 32u16);
        // D s_42_20: cast zx s_42_15 -> i
        let s_42_20: i128 = (i128::try_from(s_42_15).unwrap());
        // C s_42_21: const #1u : u64
        let s_42_21: u64 = 1;
        // D s_42_22: bit-insert s_42_19 s_42_19 s_42_20 s_42_21
        let s_42_22: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_42_21 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_42_19.length(),
            );
            (s_42_19 & mask) | (s_42_19 << s_42_20)
        };
        // D s_42_23: cast reint s_42_22 -> u32
        let s_42_23: u32 = (s_42_22.value() as u32);
        // D s_42_24: write-var match_word <= s_42_23
        fn_state.match_word = s_42_23;
        // N s_42_25: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_43_0: const #2s : i
        let s_43_0: i128 = 2;
        // D s_43_1: read-var vaddress:u32
        let s_43_1: u32 = fn_state.vaddress;
        // D s_43_2: cast zx s_43_1 -> bv
        let s_43_2: Bits = Bits::new(s_43_1 as u128, 32u16);
        // C s_43_3: const #1s : i64
        let s_43_3: i64 = 1;
        // C s_43_4: cast zx s_43_3 -> i
        let s_43_4: i128 = (i128::try_from(s_43_3).unwrap());
        // C s_43_5: const #2s : i
        let s_43_5: i128 = 2;
        // C s_43_6: add s_43_5 s_43_4
        let s_43_6: i128 = (s_43_5 + s_43_4);
        // D s_43_7: bit-extract s_43_2 s_43_0 s_43_6
        let s_43_7: Bits = (Bits::new(
            ((s_43_2) >> (s_43_0)).value(),
            u16::try_from(s_43_6).unwrap(),
        ));
        // D s_43_8: cast reint s_43_7 -> u8
        let s_43_8: u8 = (s_43_7.value() as u8);
        // D s_43_9: cast zx s_43_8 -> bv
        let s_43_9: Bits = Bits::new(s_43_8 as u128, 3u16);
        // D s_43_10: cast zx s_43_9 -> i
        let s_43_10: i128 = (s_43_9.value() as i128);
        // D s_43_11: cast reint s_43_10 -> i64
        let s_43_11: i64 = (s_43_10 as i64);
        // C s_43_12: const #24s : i
        let s_43_12: i128 = 24;
        // D s_43_13: cast zx s_43_11 -> i
        let s_43_13: i128 = (i128::try_from(s_43_11).unwrap());
        // D s_43_14: add s_43_13 s_43_12
        let s_43_14: i128 = (s_43_13 + s_43_12);
        // D s_43_15: cast reint s_43_14 -> i64
        let s_43_15: i64 = (s_43_14 as i64);
        // C s_43_16: const #1u : u8
        let s_43_16: bool = true;
        // S s_43_17: call Bit(s_43_16)
        let s_43_17: bool = Bit(state, tracer, s_43_16);
        // D s_43_18: read-var match_word:u32
        let s_43_18: u32 = fn_state.match_word;
        // D s_43_19: cast zx s_43_18 -> bv
        let s_43_19: Bits = Bits::new(s_43_18 as u128, 32u16);
        // D s_43_20: cast zx s_43_15 -> i
        let s_43_20: i128 = (i128::try_from(s_43_15).unwrap());
        // C s_43_21: const #1u : u64
        let s_43_21: u64 = 1;
        // D s_43_22: bit-insert s_43_19 s_43_19 s_43_20 s_43_21
        let s_43_22: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_43_21 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_43_19.length(),
            );
            (s_43_19 & mask) | (s_43_19 << s_43_20)
        };
        // D s_43_23: cast reint s_43_22 -> u32
        let s_43_23: u32 = (s_43_22.value() as u32);
        // D s_43_24: write-var match_word <= s_43_23
        fn_state.match_word = s_43_23;
        // N s_43_25: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_44_0: read-var ssshadow#551:u32
        let s_44_0: u32 = fn_state.ssshadow_551;
        // C s_44_1: const #0u : u32
        let s_44_1: u32 = 0;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // D s_44_3: write-var gs#30072 <= s_44_2
        fn_state.gs_30072 = s_44_2;
        // N s_44_4: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_45_0: const #16975u : u32
        let s_45_0: u32 = 16975;
        // D s_45_1: read-reg s_45_0:u8
        let s_45_1: u8 = {
            let value = state.read_register::<u8>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: cast zx s_45_1 -> bv
        let s_45_2: Bits = Bits::new(s_45_1 as u128, 2u16);
        // C s_45_3: const #432u : u32
        let s_45_3: u32 = 432;
        // D s_45_4: read-reg s_45_3:u8
        let s_45_4: u8 = {
            let value = state.read_register::<u8>(s_45_3 as isize);
            tracer.read_register(s_45_3 as isize, value);
            value
        };
        // D s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 2u16);
        // D s_45_6: cmp-ne s_45_2 s_45_5
        let s_45_6: bool = ((s_45_2) != (s_45_5));
        // D s_45_7: write-var gs#30052 <= s_45_6
        fn_state.gs_30052 = s_45_6;
        // N s_45_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_46_0: const #440u : u32
        let s_46_0: u32 = 440;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call ELUsingAArch32(s_46_1)
        let s_46_2: bool = ELUsingAArch32(state, tracer, s_46_1);
        // D s_46_3: write-var gs#30051 <= s_46_2
        fn_state.gs_30051 = s_46_2;
        // N s_46_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
