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
use Bit::*;
use ConditionSyndrome::*;
use Zeros::*;
use u__UNKNOWN_bits::*;
use ExceptionSyndrome::*;
use Unreachable::*;
use common::*;
pub fn AArch32_SystemAccessTrapSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    instr: u32,
    ec: i128,
) -> ProductTypeb7f99f96751e17c4 {
    #[derive(Default)]
    struct FunctionState {
        gs_30667: bool,
        iss: u32,
        gs_30665: bool,
        except: ProductTypeb7f99f96751e17c4,
        return_value: ProductTypeb7f99f96751e17c4,
        gs_30666: bool,
        gs_30664: bool,
        instr: u32,
        ec: i128,
    }
    let fn_state = FunctionState {
        instr,
        ec,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_0_0: read-var ec:i
        let s_0_0: i128 = fn_state.ec;
        // C s_0_1: const #0s : i
        let s_0_1: i128 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b30 b1
        if s_0_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // S s_1_1: call ExceptionSyndrome(s_1_0)
        let s_1_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_1_0);
        // D s_1_2: write-var except <= s_1_1
        fn_state.except = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_2_0: const #20s : i
        let s_2_0: i128 = 20;
        // S s_2_1: call Zeros(s_2_0)
        let s_2_1: Bits = Zeros(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u20
        let s_2_2: u32 = (s_2_1.value() as u32);
        // D s_2_3: write-var iss <= s_2_2
        fn_state.iss = s_2_2;
        // D s_2_4: read-var except.1:struct
        let s_2_4: u32 = fn_state.except._1;
        // C s_2_5: const #0u : u32
        let s_2_5: u32 = 0;
        // D s_2_6: cmp-eq s_2_4 s_2_5
        let s_2_6: bool = ((s_2_4) == (s_2_5));
        // N s_2_7: branch s_2_6 b29 b3
        if s_2_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_3_0: read-var except.1:struct
        let s_3_0: u32 = fn_state.except._1;
        // C s_3_1: const #8u : u32
        let s_3_1: u32 = 8;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b28 b4
        if s_3_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_4_0: read-var except.1:struct
        let s_4_0: u32 = fn_state.except._1;
        // C s_4_1: const #4u : u32
        let s_4_1: u32 = 4;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b27 b5
        if s_4_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_5_0: read-var except.1:struct
        let s_5_0: u32 = fn_state.except._1;
        // C s_5_1: const #2u : u32
        let s_5_1: u32 = 2;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: write-var gs#30664 <= s_5_2
        fn_state.gs_30664 = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_6_0: read-var gs#30664:u8
        let s_6_0: bool = fn_state.gs_30664;
        // D s_6_1: write-var gs#30665 <= s_6_0
        fn_state.gs_30665 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_7_0: read-var gs#30665:u8
        let s_7_0: bool = fn_state.gs_30665;
        // N s_7_1: branch s_7_0 b24 b8
        if s_7_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_8_0: read-var except.1:struct
        let s_8_0: u32 = fn_state.except._1;
        // C s_8_1: const #6u : u32
        let s_8_1: u32 = 6;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b23 b9
        if s_8_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_9_0: read-var except.1:struct
        let s_9_0: u32 = fn_state.except._1;
        // C s_9_1: const #7u : u32
        let s_9_1: u32 = 7;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b22 b10
        if s_9_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_10_0: read-var except.1:struct
        let s_10_0: u32 = fn_state.except._1;
        // C s_10_1: const #3u : u32
        let s_10_1: u32 = 3;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: write-var gs#30666 <= s_10_2
        fn_state.gs_30666 = s_10_2;
        // N s_10_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_11_0: read-var gs#30666:u8
        let s_11_0: bool = fn_state.gs_30666;
        // D s_11_1: write-var gs#30667 <= s_11_0
        fn_state.gs_30667 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_12_0: read-var gs#30667:u8
        let s_12_0: bool = fn_state.gs_30667;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_13_0: read-var except.1:struct
        let s_13_0: u32 = fn_state.except._1;
        // C s_13_1: const #5u : u32
        let s_13_1: u32 = 5;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b17 b14
        if s_13_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_15_0: const #20s : i
        let s_15_0: i128 = 20;
        // D s_15_1: read-var instr:u32
        let s_15_1: u32 = fn_state.instr;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 32u16);
        // C s_15_3: const #1u : u64
        let s_15_3: u64 = 1;
        // D s_15_4: bit-extract s_15_2 s_15_0 s_15_3
        let s_15_4: Bits = (Bits::new(
            ((s_15_2) >> (s_15_0)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_5: cast reint s_15_4 -> u8
        let s_15_5: bool = ((s_15_4.value()) != 0);
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // C s_15_7: const #0u : u64
        let s_15_7: u64 = 0;
        // D s_15_8: cast zx s_15_5 -> u64
        let s_15_8: u64 = (s_15_5 as u64);
        // C s_15_9: const #1u : u64
        let s_15_9: u64 = 1;
        // D s_15_10: and s_15_8 s_15_9
        let s_15_10: u64 = ((s_15_8) & (s_15_9));
        // D s_15_11: cmp-eq s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) == (s_15_9));
        // D s_15_12: lsl s_15_8 s_15_6
        let s_15_12: u64 = s_15_8 << s_15_6;
        // D s_15_13: or s_15_7 s_15_12
        let s_15_13: u64 = ((s_15_7) | (s_15_12));
        // D s_15_14: cmpl s_15_12
        let s_15_14: u64 = !s_15_12;
        // D s_15_15: and s_15_7 s_15_14
        let s_15_15: u64 = ((s_15_7) & (s_15_14));
        // D s_15_16: select s_15_11 s_15_13 s_15_15
        let s_15_16: u64 = if s_15_11 { s_15_13 } else { s_15_15 };
        // D s_15_17: cast trunc s_15_16 -> u8
        let s_15_17: bool = ((s_15_16) != 0);
        // D s_15_18: call Bit(s_15_17)
        let s_15_18: bool = Bit(state, tracer, s_15_17);
        // C s_15_19: const #0s : i
        let s_15_19: i128 = 0;
        // D s_15_20: read-var iss:u20
        let s_15_20: u32 = fn_state.iss;
        // D s_15_21: cast zx s_15_20 -> bv
        let s_15_21: Bits = Bits::new(s_15_20 as u128, 20u16);
        // C s_15_22: const #1u : u64
        let s_15_22: u64 = 1;
        // D s_15_23: bit-insert s_15_21 s_15_21 s_15_19 s_15_22
        let s_15_23: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_15_22 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_15_21.length(),
            );
            (s_15_21 & mask) | (s_15_21 << s_15_19)
        };
        // D s_15_24: cast reint s_15_23 -> u20
        let s_15_24: u32 = (s_15_23.value() as u32);
        // D s_15_25: write-var iss <= s_15_24
        fn_state.iss = s_15_24;
        // C s_15_26: const #() : ()
        let s_15_26: () = ();
        // S s_15_27: call ConditionSyndrome(s_15_26)
        let s_15_27: u8 = ConditionSyndrome(state, tracer, s_15_26);
        // D s_15_28: read-var except:struct
        let s_15_28: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_15_29: write-var except <= s_15_28
        fn_state.except = s_15_28;
        // D s_15_30: read-var except:struct
        let s_15_30: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_15_31: write-var except <= s_15_30
        fn_state.except = s_15_30;
        // D s_15_32: read-var except:struct
        let s_15_32: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_15_33: write-var return_value <= s_15_32
        fn_state.return_value = s_15_32;
        // N s_15_34: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_16_0: read-var return_value:struct
        let s_16_0: ProductTypeb7f99f96751e17c4 = fn_state.return_value;
        // N s_16_1: return s_16_0
        return s_16_0;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_17_0: const #0s : i
        let s_17_0: i128 = 0;
        // D s_17_1: read-var instr:u32
        let s_17_1: u32 = fn_state.instr;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 32u16);
        // C s_17_3: const #1s : i64
        let s_17_3: i64 = 1;
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #7s : i
        let s_17_5: i128 = 7;
        // C s_17_6: add s_17_5 s_17_4
        let s_17_6: i128 = (s_17_5 + s_17_4);
        // D s_17_7: bit-extract s_17_2 s_17_0 s_17_6
        let s_17_7: Bits = (Bits::new(
            ((s_17_2) >> (s_17_0)).value(),
            u16::try_from(s_17_6).unwrap(),
        ));
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: u8 = (s_17_7.value() as u8);
        // C s_17_9: const #12s : i
        let s_17_9: i128 = 12;
        // D s_17_10: read-var iss:u20
        let s_17_10: u32 = fn_state.iss;
        // D s_17_11: cast zx s_17_10 -> bv
        let s_17_11: Bits = Bits::new(s_17_10 as u128, 20u16);
        // D s_17_12: cast zx s_17_8 -> bv
        let s_17_12: Bits = Bits::new(s_17_8 as u128, 8u16);
        // C s_17_13: const #7s : i
        let s_17_13: i128 = 7;
        // C s_17_14: const #1u : u64
        let s_17_14: u64 = 1;
        // C s_17_15: cast zx s_17_14 -> bv
        let s_17_15: Bits = Bits::new(s_17_14 as u128, 64u16);
        // C s_17_16: lsl s_17_15 s_17_13
        let s_17_16: Bits = s_17_15 << s_17_13;
        // C s_17_17: sub s_17_16 s_17_15
        let s_17_17: Bits = ((s_17_16) - (s_17_15));
        // D s_17_18: and s_17_12 s_17_17
        let s_17_18: Bits = ((s_17_12) & (s_17_17));
        // D s_17_19: lsl s_17_18 s_17_9
        let s_17_19: Bits = s_17_18 << s_17_9;
        // C s_17_20: lsl s_17_17 s_17_9
        let s_17_20: Bits = s_17_17 << s_17_9;
        // C s_17_21: cmpl s_17_20
        let s_17_21: Bits = !s_17_20;
        // D s_17_22: and s_17_11 s_17_21
        let s_17_22: Bits = ((s_17_11) & (s_17_21));
        // D s_17_23: or s_17_22 s_17_19
        let s_17_23: Bits = ((s_17_22) | (s_17_19));
        // D s_17_24: cast reint s_17_23 -> u20
        let s_17_24: u32 = (s_17_23.value() as u32);
        // D s_17_25: write-var iss <= s_17_24
        fn_state.iss = s_17_24;
        // C s_17_26: const #23s : i
        let s_17_26: i128 = 23;
        // D s_17_27: read-var instr:u32
        let s_17_27: u32 = fn_state.instr;
        // D s_17_28: cast zx s_17_27 -> bv
        let s_17_28: Bits = Bits::new(s_17_27 as u128, 32u16);
        // C s_17_29: const #1u : u64
        let s_17_29: u64 = 1;
        // D s_17_30: bit-extract s_17_28 s_17_26 s_17_29
        let s_17_30: Bits = (Bits::new(
            ((s_17_28) >> (s_17_26)).value(),
            u16::try_from(s_17_29).unwrap(),
        ));
        // D s_17_31: cast reint s_17_30 -> u8
        let s_17_31: bool = ((s_17_30.value()) != 0);
        // C s_17_32: const #0s : i
        let s_17_32: i128 = 0;
        // C s_17_33: const #0u : u64
        let s_17_33: u64 = 0;
        // D s_17_34: cast zx s_17_31 -> u64
        let s_17_34: u64 = (s_17_31 as u64);
        // C s_17_35: const #1u : u64
        let s_17_35: u64 = 1;
        // D s_17_36: and s_17_34 s_17_35
        let s_17_36: u64 = ((s_17_34) & (s_17_35));
        // D s_17_37: cmp-eq s_17_36 s_17_35
        let s_17_37: bool = ((s_17_36) == (s_17_35));
        // D s_17_38: lsl s_17_34 s_17_32
        let s_17_38: u64 = s_17_34 << s_17_32;
        // D s_17_39: or s_17_33 s_17_38
        let s_17_39: u64 = ((s_17_33) | (s_17_38));
        // D s_17_40: cmpl s_17_38
        let s_17_40: u64 = !s_17_38;
        // D s_17_41: and s_17_33 s_17_40
        let s_17_41: u64 = ((s_17_33) & (s_17_40));
        // D s_17_42: select s_17_37 s_17_39 s_17_41
        let s_17_42: u64 = if s_17_37 { s_17_39 } else { s_17_41 };
        // D s_17_43: cast trunc s_17_42 -> u8
        let s_17_43: bool = ((s_17_42) != 0);
        // D s_17_44: call Bit(s_17_43)
        let s_17_44: bool = Bit(state, tracer, s_17_43);
        // C s_17_45: const #4s : i
        let s_17_45: i128 = 4;
        // D s_17_46: read-var iss:u20
        let s_17_46: u32 = fn_state.iss;
        // D s_17_47: cast zx s_17_46 -> bv
        let s_17_47: Bits = Bits::new(s_17_46 as u128, 20u16);
        // C s_17_48: const #1u : u64
        let s_17_48: u64 = 1;
        // D s_17_49: bit-insert s_17_47 s_17_47 s_17_45 s_17_48
        let s_17_49: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_17_48 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_17_47.length(),
            );
            (s_17_47 & mask) | (s_17_47 << s_17_45)
        };
        // D s_17_50: cast reint s_17_49 -> u20
        let s_17_50: u32 = (s_17_49.value() as u32);
        // D s_17_51: write-var iss <= s_17_50
        fn_state.iss = s_17_50;
        // C s_17_52: const #24s : i
        let s_17_52: i128 = 24;
        // D s_17_53: read-var instr:u32
        let s_17_53: u32 = fn_state.instr;
        // D s_17_54: cast zx s_17_53 -> bv
        let s_17_54: Bits = Bits::new(s_17_53 as u128, 32u16);
        // C s_17_55: const #1u : u64
        let s_17_55: u64 = 1;
        // D s_17_56: bit-extract s_17_54 s_17_52 s_17_55
        let s_17_56: Bits = (Bits::new(
            ((s_17_54) >> (s_17_52)).value(),
            u16::try_from(s_17_55).unwrap(),
        ));
        // D s_17_57: cast reint s_17_56 -> u8
        let s_17_57: bool = ((s_17_56.value()) != 0);
        // C s_17_58: const #0s : i
        let s_17_58: i128 = 0;
        // C s_17_59: const #0u : u64
        let s_17_59: u64 = 0;
        // D s_17_60: cast zx s_17_57 -> u64
        let s_17_60: u64 = (s_17_57 as u64);
        // C s_17_61: const #1u : u64
        let s_17_61: u64 = 1;
        // D s_17_62: and s_17_60 s_17_61
        let s_17_62: u64 = ((s_17_60) & (s_17_61));
        // D s_17_63: cmp-eq s_17_62 s_17_61
        let s_17_63: bool = ((s_17_62) == (s_17_61));
        // D s_17_64: lsl s_17_60 s_17_58
        let s_17_64: u64 = s_17_60 << s_17_58;
        // D s_17_65: or s_17_59 s_17_64
        let s_17_65: u64 = ((s_17_59) | (s_17_64));
        // D s_17_66: cmpl s_17_64
        let s_17_66: u64 = !s_17_64;
        // D s_17_67: and s_17_59 s_17_66
        let s_17_67: u64 = ((s_17_59) & (s_17_66));
        // D s_17_68: select s_17_63 s_17_65 s_17_67
        let s_17_68: u64 = if s_17_63 { s_17_65 } else { s_17_67 };
        // D s_17_69: cast trunc s_17_68 -> u8
        let s_17_69: bool = ((s_17_68) != 0);
        // C s_17_70: const #21s : i
        let s_17_70: i128 = 21;
        // D s_17_71: read-var instr:u32
        let s_17_71: u32 = fn_state.instr;
        // D s_17_72: cast zx s_17_71 -> bv
        let s_17_72: Bits = Bits::new(s_17_71 as u128, 32u16);
        // C s_17_73: const #1u : u64
        let s_17_73: u64 = 1;
        // D s_17_74: bit-extract s_17_72 s_17_70 s_17_73
        let s_17_74: Bits = (Bits::new(
            ((s_17_72) >> (s_17_70)).value(),
            u16::try_from(s_17_73).unwrap(),
        ));
        // D s_17_75: cast reint s_17_74 -> u8
        let s_17_75: bool = ((s_17_74.value()) != 0);
        // C s_17_76: const #0s : i
        let s_17_76: i128 = 0;
        // C s_17_77: const #0u : u64
        let s_17_77: u64 = 0;
        // D s_17_78: cast zx s_17_75 -> u64
        let s_17_78: u64 = (s_17_75 as u64);
        // C s_17_79: const #1u : u64
        let s_17_79: u64 = 1;
        // D s_17_80: and s_17_78 s_17_79
        let s_17_80: u64 = ((s_17_78) & (s_17_79));
        // D s_17_81: cmp-eq s_17_80 s_17_79
        let s_17_81: bool = ((s_17_80) == (s_17_79));
        // D s_17_82: lsl s_17_78 s_17_76
        let s_17_82: u64 = s_17_78 << s_17_76;
        // D s_17_83: or s_17_77 s_17_82
        let s_17_83: u64 = ((s_17_77) | (s_17_82));
        // D s_17_84: cmpl s_17_82
        let s_17_84: u64 = !s_17_82;
        // D s_17_85: and s_17_77 s_17_84
        let s_17_85: u64 = ((s_17_77) & (s_17_84));
        // D s_17_86: select s_17_81 s_17_83 s_17_85
        let s_17_86: u64 = if s_17_81 { s_17_83 } else { s_17_85 };
        // D s_17_87: cast trunc s_17_86 -> u8
        let s_17_87: bool = ((s_17_86) != 0);
        // D s_17_88: cast zx s_17_69 -> bv
        let s_17_88: Bits = Bits::new(s_17_69 as u128, 1u16);
        // D s_17_89: cast zx s_17_87 -> bv
        let s_17_89: Bits = Bits::new(s_17_87 as u128, 1u16);
        // D s_17_90: cast reint s_17_88 -> u128
        let s_17_90: u128 = (s_17_88.value() as u128);
        // D s_17_91: size-of s_17_88
        let s_17_91: u16 = s_17_88.length();
        // D s_17_92: cast reint s_17_89 -> u128
        let s_17_92: u128 = (s_17_89.value() as u128);
        // D s_17_93: size-of s_17_89
        let s_17_93: u16 = s_17_89.length();
        // D s_17_94: lsl s_17_90 s_17_93
        let s_17_94: u128 = s_17_90 << s_17_93;
        // D s_17_95: or s_17_94 s_17_92
        let s_17_95: u128 = ((s_17_94) | (s_17_92));
        // D s_17_96: add s_17_91 s_17_93
        let s_17_96: u16 = (s_17_91 + s_17_93);
        // D s_17_97: create-bits s_17_95 s_17_96
        let s_17_97: Bits = Bits::new(s_17_95, s_17_96);
        // D s_17_98: cast reint s_17_97 -> u8
        let s_17_98: u8 = (s_17_97.value() as u8);
        // C s_17_99: const #1s : i
        let s_17_99: i128 = 1;
        // D s_17_100: read-var iss:u20
        let s_17_100: u32 = fn_state.iss;
        // D s_17_101: cast zx s_17_100 -> bv
        let s_17_101: Bits = Bits::new(s_17_100 as u128, 20u16);
        // D s_17_102: cast zx s_17_98 -> bv
        let s_17_102: Bits = Bits::new(s_17_98 as u128, 2u16);
        // C s_17_103: const #1s : i
        let s_17_103: i128 = 1;
        // C s_17_104: const #1u : u64
        let s_17_104: u64 = 1;
        // C s_17_105: cast zx s_17_104 -> bv
        let s_17_105: Bits = Bits::new(s_17_104 as u128, 64u16);
        // C s_17_106: lsl s_17_105 s_17_103
        let s_17_106: Bits = s_17_105 << s_17_103;
        // C s_17_107: sub s_17_106 s_17_105
        let s_17_107: Bits = ((s_17_106) - (s_17_105));
        // D s_17_108: and s_17_102 s_17_107
        let s_17_108: Bits = ((s_17_102) & (s_17_107));
        // D s_17_109: lsl s_17_108 s_17_99
        let s_17_109: Bits = s_17_108 << s_17_99;
        // C s_17_110: lsl s_17_107 s_17_99
        let s_17_110: Bits = s_17_107 << s_17_99;
        // C s_17_111: cmpl s_17_110
        let s_17_111: Bits = !s_17_110;
        // D s_17_112: and s_17_101 s_17_111
        let s_17_112: Bits = ((s_17_101) & (s_17_111));
        // D s_17_113: or s_17_112 s_17_109
        let s_17_113: Bits = ((s_17_112) | (s_17_109));
        // D s_17_114: cast reint s_17_113 -> u20
        let s_17_114: u32 = (s_17_113.value() as u32);
        // D s_17_115: write-var iss <= s_17_114
        fn_state.iss = s_17_114;
        // C s_17_116: const #16s : i
        let s_17_116: i128 = 16;
        // D s_17_117: read-var instr:u32
        let s_17_117: u32 = fn_state.instr;
        // D s_17_118: cast zx s_17_117 -> bv
        let s_17_118: Bits = Bits::new(s_17_117 as u128, 32u16);
        // C s_17_119: const #1s : i64
        let s_17_119: i64 = 1;
        // C s_17_120: cast zx s_17_119 -> i
        let s_17_120: i128 = (i128::try_from(s_17_119).unwrap());
        // C s_17_121: const #3s : i
        let s_17_121: i128 = 3;
        // C s_17_122: add s_17_121 s_17_120
        let s_17_122: i128 = (s_17_121 + s_17_120);
        // D s_17_123: bit-extract s_17_118 s_17_116 s_17_122
        let s_17_123: Bits = (Bits::new(
            ((s_17_118) >> (s_17_116)).value(),
            u16::try_from(s_17_122).unwrap(),
        ));
        // D s_17_124: cast reint s_17_123 -> u8
        let s_17_124: u8 = (s_17_123.value() as u8);
        // D s_17_125: cast zx s_17_124 -> bv
        let s_17_125: Bits = Bits::new(s_17_124 as u128, 4u16);
        // C s_17_126: const #15u : u8
        let s_17_126: u8 = 15;
        // C s_17_127: cast zx s_17_126 -> bv
        let s_17_127: Bits = Bits::new(s_17_126 as u128, 4u16);
        // D s_17_128: cmp-eq s_17_125 s_17_127
        let s_17_128: bool = ((s_17_125) == (s_17_127));
        // N s_17_129: branch s_17_128 b20 b18
        if s_17_128 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // N s_19_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_20_0: const #4s : i64
        let s_20_0: i64 = 4;
        // C s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // S s_20_2: call __UNKNOWN_bits(s_20_1)
        let s_20_2: Bits = u__UNKNOWN_bits(state, tracer, s_20_1);
        // S s_20_3: cast reint s_20_2 -> u8
        let s_20_3: u8 = (s_20_2.value() as u8);
        // C s_20_4: const #5s : i
        let s_20_4: i128 = 5;
        // D s_20_5: read-var iss:u20
        let s_20_5: u32 = fn_state.iss;
        // D s_20_6: cast zx s_20_5 -> bv
        let s_20_6: Bits = Bits::new(s_20_5 as u128, 20u16);
        // S s_20_7: cast zx s_20_3 -> bv
        let s_20_7: Bits = Bits::new(s_20_3 as u128, 4u16);
        // C s_20_8: const #3s : i
        let s_20_8: i128 = 3;
        // C s_20_9: const #1u : u64
        let s_20_9: u64 = 1;
        // C s_20_10: cast zx s_20_9 -> bv
        let s_20_10: Bits = Bits::new(s_20_9 as u128, 64u16);
        // C s_20_11: lsl s_20_10 s_20_8
        let s_20_11: Bits = s_20_10 << s_20_8;
        // C s_20_12: sub s_20_11 s_20_10
        let s_20_12: Bits = ((s_20_11) - (s_20_10));
        // S s_20_13: and s_20_7 s_20_12
        let s_20_13: Bits = ((s_20_7) & (s_20_12));
        // S s_20_14: lsl s_20_13 s_20_4
        let s_20_14: Bits = s_20_13 << s_20_4;
        // C s_20_15: lsl s_20_12 s_20_4
        let s_20_15: Bits = s_20_12 << s_20_4;
        // C s_20_16: cmpl s_20_15
        let s_20_16: Bits = !s_20_15;
        // D s_20_17: and s_20_6 s_20_16
        let s_20_17: Bits = ((s_20_6) & (s_20_16));
        // D s_20_18: or s_20_17 s_20_14
        let s_20_18: Bits = ((s_20_17) | (s_20_14));
        // D s_20_19: cast reint s_20_18 -> u20
        let s_20_19: u32 = (s_20_18.value() as u32);
        // D s_20_20: write-var iss <= s_20_19
        fn_state.iss = s_20_19;
        // C s_20_21: const #1u : u8
        let s_20_21: bool = true;
        // S s_20_22: call Bit(s_20_21)
        let s_20_22: bool = Bit(state, tracer, s_20_21);
        // C s_20_23: const #3s : i
        let s_20_23: i128 = 3;
        // D s_20_24: read-var iss:u20
        let s_20_24: u32 = fn_state.iss;
        // D s_20_25: cast zx s_20_24 -> bv
        let s_20_25: Bits = Bits::new(s_20_24 as u128, 20u16);
        // C s_20_26: const #1u : u64
        let s_20_26: u64 = 1;
        // D s_20_27: bit-insert s_20_25 s_20_25 s_20_23 s_20_26
        let s_20_27: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_20_26 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_20_25.length(),
            );
            (s_20_25 & mask) | (s_20_25 << s_20_23)
        };
        // D s_20_28: cast reint s_20_27 -> u20
        let s_20_28: u32 = (s_20_27.value() as u32);
        // D s_20_29: write-var iss <= s_20_28
        fn_state.iss = s_20_28;
        // N s_20_30: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_21_0: const #4s : i
        let s_21_0: i128 = 4;
        // D s_21_1: read-var instr:u32
        let s_21_1: u32 = fn_state.instr;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 32u16);
        // C s_21_3: const #1s : i64
        let s_21_3: i64 = 1;
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // C s_21_5: const #3s : i
        let s_21_5: i128 = 3;
        // C s_21_6: add s_21_5 s_21_4
        let s_21_6: i128 = (s_21_5 + s_21_4);
        // D s_21_7: bit-extract s_21_2 s_21_0 s_21_6
        let s_21_7: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_6).unwrap(),
        ));
        // D s_21_8: cast reint s_21_7 -> u8
        let s_21_8: u8 = (s_21_7.value() as u8);
        // C s_21_9: const #16s : i
        let s_21_9: i128 = 16;
        // D s_21_10: read-var iss:u20
        let s_21_10: u32 = fn_state.iss;
        // D s_21_11: cast zx s_21_10 -> bv
        let s_21_11: Bits = Bits::new(s_21_10 as u128, 20u16);
        // D s_21_12: cast zx s_21_8 -> bv
        let s_21_12: Bits = Bits::new(s_21_8 as u128, 4u16);
        // C s_21_13: const #3s : i
        let s_21_13: i128 = 3;
        // C s_21_14: const #1u : u64
        let s_21_14: u64 = 1;
        // C s_21_15: cast zx s_21_14 -> bv
        let s_21_15: Bits = Bits::new(s_21_14 as u128, 64u16);
        // C s_21_16: lsl s_21_15 s_21_13
        let s_21_16: Bits = s_21_15 << s_21_13;
        // C s_21_17: sub s_21_16 s_21_15
        let s_21_17: Bits = ((s_21_16) - (s_21_15));
        // D s_21_18: and s_21_12 s_21_17
        let s_21_18: Bits = ((s_21_12) & (s_21_17));
        // D s_21_19: lsl s_21_18 s_21_9
        let s_21_19: Bits = s_21_18 << s_21_9;
        // C s_21_20: lsl s_21_17 s_21_9
        let s_21_20: Bits = s_21_17 << s_21_9;
        // C s_21_21: cmpl s_21_20
        let s_21_21: Bits = !s_21_20;
        // D s_21_22: and s_21_11 s_21_21
        let s_21_22: Bits = ((s_21_11) & (s_21_21));
        // D s_21_23: or s_21_22 s_21_19
        let s_21_23: Bits = ((s_21_22) | (s_21_19));
        // D s_21_24: cast reint s_21_23 -> u20
        let s_21_24: u32 = (s_21_23.value() as u32);
        // D s_21_25: write-var iss <= s_21_24
        fn_state.iss = s_21_24;
        // C s_21_26: const #16s : i
        let s_21_26: i128 = 16;
        // D s_21_27: read-var instr:u32
        let s_21_27: u32 = fn_state.instr;
        // D s_21_28: cast zx s_21_27 -> bv
        let s_21_28: Bits = Bits::new(s_21_27 as u128, 32u16);
        // C s_21_29: const #1s : i64
        let s_21_29: i64 = 1;
        // C s_21_30: cast zx s_21_29 -> i
        let s_21_30: i128 = (i128::try_from(s_21_29).unwrap());
        // C s_21_31: const #3s : i
        let s_21_31: i128 = 3;
        // C s_21_32: add s_21_31 s_21_30
        let s_21_32: i128 = (s_21_31 + s_21_30);
        // D s_21_33: bit-extract s_21_28 s_21_26 s_21_32
        let s_21_33: Bits = (Bits::new(
            ((s_21_28) >> (s_21_26)).value(),
            u16::try_from(s_21_32).unwrap(),
        ));
        // D s_21_34: cast reint s_21_33 -> u8
        let s_21_34: u8 = (s_21_33.value() as u8);
        // C s_21_35: const #10s : i
        let s_21_35: i128 = 10;
        // D s_21_36: read-var iss:u20
        let s_21_36: u32 = fn_state.iss;
        // D s_21_37: cast zx s_21_36 -> bv
        let s_21_37: Bits = Bits::new(s_21_36 as u128, 20u16);
        // D s_21_38: cast zx s_21_34 -> bv
        let s_21_38: Bits = Bits::new(s_21_34 as u128, 4u16);
        // C s_21_39: const #3s : i
        let s_21_39: i128 = 3;
        // C s_21_40: const #1u : u64
        let s_21_40: u64 = 1;
        // C s_21_41: cast zx s_21_40 -> bv
        let s_21_41: Bits = Bits::new(s_21_40 as u128, 64u16);
        // C s_21_42: lsl s_21_41 s_21_39
        let s_21_42: Bits = s_21_41 << s_21_39;
        // C s_21_43: sub s_21_42 s_21_41
        let s_21_43: Bits = ((s_21_42) - (s_21_41));
        // D s_21_44: and s_21_38 s_21_43
        let s_21_44: Bits = ((s_21_38) & (s_21_43));
        // D s_21_45: lsl s_21_44 s_21_35
        let s_21_45: Bits = s_21_44 << s_21_35;
        // C s_21_46: lsl s_21_43 s_21_35
        let s_21_46: Bits = s_21_43 << s_21_35;
        // C s_21_47: cmpl s_21_46
        let s_21_47: Bits = !s_21_46;
        // D s_21_48: and s_21_37 s_21_47
        let s_21_48: Bits = ((s_21_37) & (s_21_47));
        // D s_21_49: or s_21_48 s_21_45
        let s_21_49: Bits = ((s_21_48) | (s_21_45));
        // D s_21_50: cast reint s_21_49 -> u20
        let s_21_50: u32 = (s_21_49.value() as u32);
        // D s_21_51: write-var iss <= s_21_50
        fn_state.iss = s_21_50;
        // C s_21_52: const #12s : i
        let s_21_52: i128 = 12;
        // D s_21_53: read-var instr:u32
        let s_21_53: u32 = fn_state.instr;
        // D s_21_54: cast zx s_21_53 -> bv
        let s_21_54: Bits = Bits::new(s_21_53 as u128, 32u16);
        // C s_21_55: const #1s : i64
        let s_21_55: i64 = 1;
        // C s_21_56: cast zx s_21_55 -> i
        let s_21_56: i128 = (i128::try_from(s_21_55).unwrap());
        // C s_21_57: const #3s : i
        let s_21_57: i128 = 3;
        // C s_21_58: add s_21_57 s_21_56
        let s_21_58: i128 = (s_21_57 + s_21_56);
        // D s_21_59: bit-extract s_21_54 s_21_52 s_21_58
        let s_21_59: Bits = (Bits::new(
            ((s_21_54) >> (s_21_52)).value(),
            u16::try_from(s_21_58).unwrap(),
        ));
        // D s_21_60: cast reint s_21_59 -> u8
        let s_21_60: u8 = (s_21_59.value() as u8);
        // C s_21_61: const #5s : i
        let s_21_61: i128 = 5;
        // D s_21_62: read-var iss:u20
        let s_21_62: u32 = fn_state.iss;
        // D s_21_63: cast zx s_21_62 -> bv
        let s_21_63: Bits = Bits::new(s_21_62 as u128, 20u16);
        // D s_21_64: cast zx s_21_60 -> bv
        let s_21_64: Bits = Bits::new(s_21_60 as u128, 4u16);
        // C s_21_65: const #3s : i
        let s_21_65: i128 = 3;
        // C s_21_66: const #1u : u64
        let s_21_66: u64 = 1;
        // C s_21_67: cast zx s_21_66 -> bv
        let s_21_67: Bits = Bits::new(s_21_66 as u128, 64u16);
        // C s_21_68: lsl s_21_67 s_21_65
        let s_21_68: Bits = s_21_67 << s_21_65;
        // C s_21_69: sub s_21_68 s_21_67
        let s_21_69: Bits = ((s_21_68) - (s_21_67));
        // D s_21_70: and s_21_64 s_21_69
        let s_21_70: Bits = ((s_21_64) & (s_21_69));
        // D s_21_71: lsl s_21_70 s_21_61
        let s_21_71: Bits = s_21_70 << s_21_61;
        // C s_21_72: lsl s_21_69 s_21_61
        let s_21_72: Bits = s_21_69 << s_21_61;
        // C s_21_73: cmpl s_21_72
        let s_21_73: Bits = !s_21_72;
        // D s_21_74: and s_21_63 s_21_73
        let s_21_74: Bits = ((s_21_63) & (s_21_73));
        // D s_21_75: or s_21_74 s_21_71
        let s_21_75: Bits = ((s_21_74) | (s_21_71));
        // D s_21_76: cast reint s_21_75 -> u20
        let s_21_76: u32 = (s_21_75.value() as u32);
        // D s_21_77: write-var iss <= s_21_76
        fn_state.iss = s_21_76;
        // C s_21_78: const #0s : i
        let s_21_78: i128 = 0;
        // D s_21_79: read-var instr:u32
        let s_21_79: u32 = fn_state.instr;
        // D s_21_80: cast zx s_21_79 -> bv
        let s_21_80: Bits = Bits::new(s_21_79 as u128, 32u16);
        // C s_21_81: const #1s : i64
        let s_21_81: i64 = 1;
        // C s_21_82: cast zx s_21_81 -> i
        let s_21_82: i128 = (i128::try_from(s_21_81).unwrap());
        // C s_21_83: const #3s : i
        let s_21_83: i128 = 3;
        // C s_21_84: add s_21_83 s_21_82
        let s_21_84: i128 = (s_21_83 + s_21_82);
        // D s_21_85: bit-extract s_21_80 s_21_78 s_21_84
        let s_21_85: Bits = (Bits::new(
            ((s_21_80) >> (s_21_78)).value(),
            u16::try_from(s_21_84).unwrap(),
        ));
        // D s_21_86: cast reint s_21_85 -> u8
        let s_21_86: u8 = (s_21_85.value() as u8);
        // C s_21_87: const #1s : i
        let s_21_87: i128 = 1;
        // D s_21_88: read-var iss:u20
        let s_21_88: u32 = fn_state.iss;
        // D s_21_89: cast zx s_21_88 -> bv
        let s_21_89: Bits = Bits::new(s_21_88 as u128, 20u16);
        // D s_21_90: cast zx s_21_86 -> bv
        let s_21_90: Bits = Bits::new(s_21_86 as u128, 4u16);
        // C s_21_91: const #3s : i
        let s_21_91: i128 = 3;
        // C s_21_92: const #1u : u64
        let s_21_92: u64 = 1;
        // C s_21_93: cast zx s_21_92 -> bv
        let s_21_93: Bits = Bits::new(s_21_92 as u128, 64u16);
        // C s_21_94: lsl s_21_93 s_21_91
        let s_21_94: Bits = s_21_93 << s_21_91;
        // C s_21_95: sub s_21_94 s_21_93
        let s_21_95: Bits = ((s_21_94) - (s_21_93));
        // D s_21_96: and s_21_90 s_21_95
        let s_21_96: Bits = ((s_21_90) & (s_21_95));
        // D s_21_97: lsl s_21_96 s_21_87
        let s_21_97: Bits = s_21_96 << s_21_87;
        // C s_21_98: lsl s_21_95 s_21_87
        let s_21_98: Bits = s_21_95 << s_21_87;
        // C s_21_99: cmpl s_21_98
        let s_21_99: Bits = !s_21_98;
        // D s_21_100: and s_21_89 s_21_99
        let s_21_100: Bits = ((s_21_89) & (s_21_99));
        // D s_21_101: or s_21_100 s_21_97
        let s_21_101: Bits = ((s_21_100) | (s_21_97));
        // D s_21_102: cast reint s_21_101 -> u20
        let s_21_102: u32 = (s_21_101.value() as u32);
        // D s_21_103: write-var iss <= s_21_102
        fn_state.iss = s_21_102;
        // N s_21_104: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#30666 <= s_22_0
        fn_state.gs_30666 = s_22_0;
        // N s_22_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#30667 <= s_23_0
        fn_state.gs_30667 = s_23_0;
        // N s_23_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_24_0: const #16s : i
        let s_24_0: i128 = 16;
        // D s_24_1: read-var instr:u32
        let s_24_1: u32 = fn_state.instr;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 32u16);
        // C s_24_3: const #1s : i64
        let s_24_3: i64 = 1;
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #3s : i
        let s_24_5: i128 = 3;
        // C s_24_6: add s_24_5 s_24_4
        let s_24_6: i128 = (s_24_5 + s_24_4);
        // D s_24_7: bit-extract s_24_2 s_24_0 s_24_6
        let s_24_7: Bits = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_6).unwrap(),
        ));
        // D s_24_8: cast reint s_24_7 -> u8
        let s_24_8: u8 = (s_24_7.value() as u8);
        // C s_24_9: const #10s : i
        let s_24_9: i128 = 10;
        // D s_24_10: read-var iss:u20
        let s_24_10: u32 = fn_state.iss;
        // D s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 20u16);
        // D s_24_12: cast zx s_24_8 -> bv
        let s_24_12: Bits = Bits::new(s_24_8 as u128, 4u16);
        // C s_24_13: const #3s : i
        let s_24_13: i128 = 3;
        // C s_24_14: const #1u : u64
        let s_24_14: u64 = 1;
        // C s_24_15: cast zx s_24_14 -> bv
        let s_24_15: Bits = Bits::new(s_24_14 as u128, 64u16);
        // C s_24_16: lsl s_24_15 s_24_13
        let s_24_16: Bits = s_24_15 << s_24_13;
        // C s_24_17: sub s_24_16 s_24_15
        let s_24_17: Bits = ((s_24_16) - (s_24_15));
        // D s_24_18: and s_24_12 s_24_17
        let s_24_18: Bits = ((s_24_12) & (s_24_17));
        // D s_24_19: lsl s_24_18 s_24_9
        let s_24_19: Bits = s_24_18 << s_24_9;
        // C s_24_20: lsl s_24_17 s_24_9
        let s_24_20: Bits = s_24_17 << s_24_9;
        // C s_24_21: cmpl s_24_20
        let s_24_21: Bits = !s_24_20;
        // D s_24_22: and s_24_11 s_24_21
        let s_24_22: Bits = ((s_24_11) & (s_24_21));
        // D s_24_23: or s_24_22 s_24_19
        let s_24_23: Bits = ((s_24_22) | (s_24_19));
        // D s_24_24: cast reint s_24_23 -> u20
        let s_24_24: u32 = (s_24_23.value() as u32);
        // D s_24_25: write-var iss <= s_24_24
        fn_state.iss = s_24_24;
        // C s_24_26: const #12s : i
        let s_24_26: i128 = 12;
        // D s_24_27: read-var instr:u32
        let s_24_27: u32 = fn_state.instr;
        // D s_24_28: cast zx s_24_27 -> bv
        let s_24_28: Bits = Bits::new(s_24_27 as u128, 32u16);
        // C s_24_29: const #1s : i64
        let s_24_29: i64 = 1;
        // C s_24_30: cast zx s_24_29 -> i
        let s_24_30: i128 = (i128::try_from(s_24_29).unwrap());
        // C s_24_31: const #3s : i
        let s_24_31: i128 = 3;
        // C s_24_32: add s_24_31 s_24_30
        let s_24_32: i128 = (s_24_31 + s_24_30);
        // D s_24_33: bit-extract s_24_28 s_24_26 s_24_32
        let s_24_33: Bits = (Bits::new(
            ((s_24_28) >> (s_24_26)).value(),
            u16::try_from(s_24_32).unwrap(),
        ));
        // D s_24_34: cast reint s_24_33 -> u8
        let s_24_34: u8 = (s_24_33.value() as u8);
        // C s_24_35: const #5s : i
        let s_24_35: i128 = 5;
        // D s_24_36: read-var iss:u20
        let s_24_36: u32 = fn_state.iss;
        // D s_24_37: cast zx s_24_36 -> bv
        let s_24_37: Bits = Bits::new(s_24_36 as u128, 20u16);
        // D s_24_38: cast zx s_24_34 -> bv
        let s_24_38: Bits = Bits::new(s_24_34 as u128, 4u16);
        // C s_24_39: const #3s : i
        let s_24_39: i128 = 3;
        // C s_24_40: const #1u : u64
        let s_24_40: u64 = 1;
        // C s_24_41: cast zx s_24_40 -> bv
        let s_24_41: Bits = Bits::new(s_24_40 as u128, 64u16);
        // C s_24_42: lsl s_24_41 s_24_39
        let s_24_42: Bits = s_24_41 << s_24_39;
        // C s_24_43: sub s_24_42 s_24_41
        let s_24_43: Bits = ((s_24_42) - (s_24_41));
        // D s_24_44: and s_24_38 s_24_43
        let s_24_44: Bits = ((s_24_38) & (s_24_43));
        // D s_24_45: lsl s_24_44 s_24_35
        let s_24_45: Bits = s_24_44 << s_24_35;
        // C s_24_46: lsl s_24_43 s_24_35
        let s_24_46: Bits = s_24_43 << s_24_35;
        // C s_24_47: cmpl s_24_46
        let s_24_47: Bits = !s_24_46;
        // D s_24_48: and s_24_37 s_24_47
        let s_24_48: Bits = ((s_24_37) & (s_24_47));
        // D s_24_49: or s_24_48 s_24_45
        let s_24_49: Bits = ((s_24_48) | (s_24_45));
        // D s_24_50: cast reint s_24_49 -> u20
        let s_24_50: u32 = (s_24_49.value() as u32);
        // D s_24_51: write-var iss <= s_24_50
        fn_state.iss = s_24_50;
        // C s_24_52: const #0u : u8
        let s_24_52: bool = false;
        // S s_24_53: call Bit(s_24_52)
        let s_24_53: bool = Bit(state, tracer, s_24_52);
        // C s_24_54: const #9s : i
        let s_24_54: i128 = 9;
        // D s_24_55: read-var iss:u20
        let s_24_55: u32 = fn_state.iss;
        // D s_24_56: cast zx s_24_55 -> bv
        let s_24_56: Bits = Bits::new(s_24_55 as u128, 20u16);
        // C s_24_57: const #1u : u64
        let s_24_57: u64 = 1;
        // D s_24_58: bit-insert s_24_56 s_24_56 s_24_54 s_24_57
        let s_24_58: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_24_57 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_24_56.length(),
            );
            (s_24_56 & mask) | (s_24_56 << s_24_54)
        };
        // D s_24_59: cast reint s_24_58 -> u20
        let s_24_59: u32 = (s_24_58.value() as u32);
        // D s_24_60: write-var iss <= s_24_59
        fn_state.iss = s_24_59;
        // D s_24_61: read-var except.1:struct
        let s_24_61: u32 = fn_state.except._1;
        // C s_24_62: const #8u : u32
        let s_24_62: u32 = 8;
        // D s_24_63: cmp-eq s_24_61 s_24_62
        let s_24_63: bool = ((s_24_61) == (s_24_62));
        // N s_24_64: branch s_24_63 b26 b25
        if s_24_63 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_25_0: const #17s : i
        let s_25_0: i128 = 17;
        // D s_25_1: read-var iss:u20
        let s_25_1: u32 = fn_state.iss;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 20u16);
        // C s_25_3: const #0u : u8
        let s_25_3: u8 = 0;
        // C s_25_4: cast zx s_25_3 -> bv
        let s_25_4: Bits = Bits::new(s_25_3 as u128, 3u16);
        // C s_25_5: const #2s : i
        let s_25_5: i128 = 2;
        // C s_25_6: const #1u : u64
        let s_25_6: u64 = 1;
        // C s_25_7: cast zx s_25_6 -> bv
        let s_25_7: Bits = Bits::new(s_25_6 as u128, 64u16);
        // C s_25_8: lsl s_25_7 s_25_5
        let s_25_8: Bits = s_25_7 << s_25_5;
        // C s_25_9: sub s_25_8 s_25_7
        let s_25_9: Bits = ((s_25_8) - (s_25_7));
        // C s_25_10: and s_25_4 s_25_9
        let s_25_10: Bits = ((s_25_4) & (s_25_9));
        // C s_25_11: lsl s_25_10 s_25_0
        let s_25_11: Bits = s_25_10 << s_25_0;
        // C s_25_12: lsl s_25_9 s_25_0
        let s_25_12: Bits = s_25_9 << s_25_0;
        // C s_25_13: cmpl s_25_12
        let s_25_13: Bits = !s_25_12;
        // D s_25_14: and s_25_2 s_25_13
        let s_25_14: Bits = ((s_25_2) & (s_25_13));
        // D s_25_15: or s_25_14 s_25_11
        let s_25_15: Bits = ((s_25_14) | (s_25_11));
        // D s_25_16: cast reint s_25_15 -> u20
        let s_25_16: u32 = (s_25_15.value() as u32);
        // D s_25_17: write-var iss <= s_25_16
        fn_state.iss = s_25_16;
        // C s_25_18: const #14s : i
        let s_25_18: i128 = 14;
        // D s_25_19: read-var iss:u20
        let s_25_19: u32 = fn_state.iss;
        // D s_25_20: cast zx s_25_19 -> bv
        let s_25_20: Bits = Bits::new(s_25_19 as u128, 20u16);
        // C s_25_21: const #7u : u8
        let s_25_21: u8 = 7;
        // C s_25_22: cast zx s_25_21 -> bv
        let s_25_22: Bits = Bits::new(s_25_21 as u128, 3u16);
        // C s_25_23: const #2s : i
        let s_25_23: i128 = 2;
        // C s_25_24: const #1u : u64
        let s_25_24: u64 = 1;
        // C s_25_25: cast zx s_25_24 -> bv
        let s_25_25: Bits = Bits::new(s_25_24 as u128, 64u16);
        // C s_25_26: lsl s_25_25 s_25_23
        let s_25_26: Bits = s_25_25 << s_25_23;
        // C s_25_27: sub s_25_26 s_25_25
        let s_25_27: Bits = ((s_25_26) - (s_25_25));
        // C s_25_28: and s_25_22 s_25_27
        let s_25_28: Bits = ((s_25_22) & (s_25_27));
        // C s_25_29: lsl s_25_28 s_25_18
        let s_25_29: Bits = s_25_28 << s_25_18;
        // C s_25_30: lsl s_25_27 s_25_18
        let s_25_30: Bits = s_25_27 << s_25_18;
        // C s_25_31: cmpl s_25_30
        let s_25_31: Bits = !s_25_30;
        // D s_25_32: and s_25_20 s_25_31
        let s_25_32: Bits = ((s_25_20) & (s_25_31));
        // D s_25_33: or s_25_32 s_25_29
        let s_25_33: Bits = ((s_25_32) | (s_25_29));
        // D s_25_34: cast reint s_25_33 -> u20
        let s_25_34: u32 = (s_25_33.value() as u32);
        // D s_25_35: write-var iss <= s_25_34
        fn_state.iss = s_25_34;
        // C s_25_36: const #1s : i
        let s_25_36: i128 = 1;
        // D s_25_37: read-var iss:u20
        let s_25_37: u32 = fn_state.iss;
        // D s_25_38: cast zx s_25_37 -> bv
        let s_25_38: Bits = Bits::new(s_25_37 as u128, 20u16);
        // C s_25_39: const #0u : u8
        let s_25_39: u8 = 0;
        // C s_25_40: cast zx s_25_39 -> bv
        let s_25_40: Bits = Bits::new(s_25_39 as u128, 4u16);
        // C s_25_41: const #3s : i
        let s_25_41: i128 = 3;
        // C s_25_42: const #1u : u64
        let s_25_42: u64 = 1;
        // C s_25_43: cast zx s_25_42 -> bv
        let s_25_43: Bits = Bits::new(s_25_42 as u128, 64u16);
        // C s_25_44: lsl s_25_43 s_25_41
        let s_25_44: Bits = s_25_43 << s_25_41;
        // C s_25_45: sub s_25_44 s_25_43
        let s_25_45: Bits = ((s_25_44) - (s_25_43));
        // C s_25_46: and s_25_40 s_25_45
        let s_25_46: Bits = ((s_25_40) & (s_25_45));
        // C s_25_47: lsl s_25_46 s_25_36
        let s_25_47: Bits = s_25_46 << s_25_36;
        // C s_25_48: lsl s_25_45 s_25_36
        let s_25_48: Bits = s_25_45 << s_25_36;
        // C s_25_49: cmpl s_25_48
        let s_25_49: Bits = !s_25_48;
        // D s_25_50: and s_25_38 s_25_49
        let s_25_50: Bits = ((s_25_38) & (s_25_49));
        // D s_25_51: or s_25_50 s_25_47
        let s_25_51: Bits = ((s_25_50) | (s_25_47));
        // D s_25_52: cast reint s_25_51 -> u20
        let s_25_52: u32 = (s_25_51.value() as u32);
        // D s_25_53: write-var iss <= s_25_52
        fn_state.iss = s_25_52;
        // N s_25_54: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_26_0: const #5s : i
        let s_26_0: i128 = 5;
        // D s_26_1: read-var instr:u32
        let s_26_1: u32 = fn_state.instr;
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 32u16);
        // C s_26_3: const #1s : i64
        let s_26_3: i64 = 1;
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #2s : i
        let s_26_5: i128 = 2;
        // C s_26_6: add s_26_5 s_26_4
        let s_26_6: i128 = (s_26_5 + s_26_4);
        // D s_26_7: bit-extract s_26_2 s_26_0 s_26_6
        let s_26_7: Bits = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_6).unwrap(),
        ));
        // D s_26_8: cast reint s_26_7 -> u8
        let s_26_8: u8 = (s_26_7.value() as u8);
        // C s_26_9: const #17s : i
        let s_26_9: i128 = 17;
        // D s_26_10: read-var iss:u20
        let s_26_10: u32 = fn_state.iss;
        // D s_26_11: cast zx s_26_10 -> bv
        let s_26_11: Bits = Bits::new(s_26_10 as u128, 20u16);
        // D s_26_12: cast zx s_26_8 -> bv
        let s_26_12: Bits = Bits::new(s_26_8 as u128, 3u16);
        // C s_26_13: const #2s : i
        let s_26_13: i128 = 2;
        // C s_26_14: const #1u : u64
        let s_26_14: u64 = 1;
        // C s_26_15: cast zx s_26_14 -> bv
        let s_26_15: Bits = Bits::new(s_26_14 as u128, 64u16);
        // C s_26_16: lsl s_26_15 s_26_13
        let s_26_16: Bits = s_26_15 << s_26_13;
        // C s_26_17: sub s_26_16 s_26_15
        let s_26_17: Bits = ((s_26_16) - (s_26_15));
        // D s_26_18: and s_26_12 s_26_17
        let s_26_18: Bits = ((s_26_12) & (s_26_17));
        // D s_26_19: lsl s_26_18 s_26_9
        let s_26_19: Bits = s_26_18 << s_26_9;
        // C s_26_20: lsl s_26_17 s_26_9
        let s_26_20: Bits = s_26_17 << s_26_9;
        // C s_26_21: cmpl s_26_20
        let s_26_21: Bits = !s_26_20;
        // D s_26_22: and s_26_11 s_26_21
        let s_26_22: Bits = ((s_26_11) & (s_26_21));
        // D s_26_23: or s_26_22 s_26_19
        let s_26_23: Bits = ((s_26_22) | (s_26_19));
        // D s_26_24: cast reint s_26_23 -> u20
        let s_26_24: u32 = (s_26_23.value() as u32);
        // D s_26_25: write-var iss <= s_26_24
        fn_state.iss = s_26_24;
        // C s_26_26: const #21s : i
        let s_26_26: i128 = 21;
        // D s_26_27: read-var instr:u32
        let s_26_27: u32 = fn_state.instr;
        // D s_26_28: cast zx s_26_27 -> bv
        let s_26_28: Bits = Bits::new(s_26_27 as u128, 32u16);
        // C s_26_29: const #1s : i64
        let s_26_29: i64 = 1;
        // C s_26_30: cast zx s_26_29 -> i
        let s_26_30: i128 = (i128::try_from(s_26_29).unwrap());
        // C s_26_31: const #2s : i
        let s_26_31: i128 = 2;
        // C s_26_32: add s_26_31 s_26_30
        let s_26_32: i128 = (s_26_31 + s_26_30);
        // D s_26_33: bit-extract s_26_28 s_26_26 s_26_32
        let s_26_33: Bits = (Bits::new(
            ((s_26_28) >> (s_26_26)).value(),
            u16::try_from(s_26_32).unwrap(),
        ));
        // D s_26_34: cast reint s_26_33 -> u8
        let s_26_34: u8 = (s_26_33.value() as u8);
        // C s_26_35: const #14s : i
        let s_26_35: i128 = 14;
        // D s_26_36: read-var iss:u20
        let s_26_36: u32 = fn_state.iss;
        // D s_26_37: cast zx s_26_36 -> bv
        let s_26_37: Bits = Bits::new(s_26_36 as u128, 20u16);
        // D s_26_38: cast zx s_26_34 -> bv
        let s_26_38: Bits = Bits::new(s_26_34 as u128, 3u16);
        // C s_26_39: const #2s : i
        let s_26_39: i128 = 2;
        // C s_26_40: const #1u : u64
        let s_26_40: u64 = 1;
        // C s_26_41: cast zx s_26_40 -> bv
        let s_26_41: Bits = Bits::new(s_26_40 as u128, 64u16);
        // C s_26_42: lsl s_26_41 s_26_39
        let s_26_42: Bits = s_26_41 << s_26_39;
        // C s_26_43: sub s_26_42 s_26_41
        let s_26_43: Bits = ((s_26_42) - (s_26_41));
        // D s_26_44: and s_26_38 s_26_43
        let s_26_44: Bits = ((s_26_38) & (s_26_43));
        // D s_26_45: lsl s_26_44 s_26_35
        let s_26_45: Bits = s_26_44 << s_26_35;
        // C s_26_46: lsl s_26_43 s_26_35
        let s_26_46: Bits = s_26_43 << s_26_35;
        // C s_26_47: cmpl s_26_46
        let s_26_47: Bits = !s_26_46;
        // D s_26_48: and s_26_37 s_26_47
        let s_26_48: Bits = ((s_26_37) & (s_26_47));
        // D s_26_49: or s_26_48 s_26_45
        let s_26_49: Bits = ((s_26_48) | (s_26_45));
        // D s_26_50: cast reint s_26_49 -> u20
        let s_26_50: u32 = (s_26_49.value() as u32);
        // D s_26_51: write-var iss <= s_26_50
        fn_state.iss = s_26_50;
        // C s_26_52: const #0s : i
        let s_26_52: i128 = 0;
        // D s_26_53: read-var instr:u32
        let s_26_53: u32 = fn_state.instr;
        // D s_26_54: cast zx s_26_53 -> bv
        let s_26_54: Bits = Bits::new(s_26_53 as u128, 32u16);
        // C s_26_55: const #1s : i64
        let s_26_55: i64 = 1;
        // C s_26_56: cast zx s_26_55 -> i
        let s_26_56: i128 = (i128::try_from(s_26_55).unwrap());
        // C s_26_57: const #3s : i
        let s_26_57: i128 = 3;
        // C s_26_58: add s_26_57 s_26_56
        let s_26_58: i128 = (s_26_57 + s_26_56);
        // D s_26_59: bit-extract s_26_54 s_26_52 s_26_58
        let s_26_59: Bits = (Bits::new(
            ((s_26_54) >> (s_26_52)).value(),
            u16::try_from(s_26_58).unwrap(),
        ));
        // D s_26_60: cast reint s_26_59 -> u8
        let s_26_60: u8 = (s_26_59.value() as u8);
        // C s_26_61: const #1s : i
        let s_26_61: i128 = 1;
        // D s_26_62: read-var iss:u20
        let s_26_62: u32 = fn_state.iss;
        // D s_26_63: cast zx s_26_62 -> bv
        let s_26_63: Bits = Bits::new(s_26_62 as u128, 20u16);
        // D s_26_64: cast zx s_26_60 -> bv
        let s_26_64: Bits = Bits::new(s_26_60 as u128, 4u16);
        // C s_26_65: const #3s : i
        let s_26_65: i128 = 3;
        // C s_26_66: const #1u : u64
        let s_26_66: u64 = 1;
        // C s_26_67: cast zx s_26_66 -> bv
        let s_26_67: Bits = Bits::new(s_26_66 as u128, 64u16);
        // C s_26_68: lsl s_26_67 s_26_65
        let s_26_68: Bits = s_26_67 << s_26_65;
        // C s_26_69: sub s_26_68 s_26_67
        let s_26_69: Bits = ((s_26_68) - (s_26_67));
        // D s_26_70: and s_26_64 s_26_69
        let s_26_70: Bits = ((s_26_64) & (s_26_69));
        // D s_26_71: lsl s_26_70 s_26_61
        let s_26_71: Bits = s_26_70 << s_26_61;
        // C s_26_72: lsl s_26_69 s_26_61
        let s_26_72: Bits = s_26_69 << s_26_61;
        // C s_26_73: cmpl s_26_72
        let s_26_73: Bits = !s_26_72;
        // D s_26_74: and s_26_63 s_26_73
        let s_26_74: Bits = ((s_26_63) & (s_26_73));
        // D s_26_75: or s_26_74 s_26_71
        let s_26_75: Bits = ((s_26_74) | (s_26_71));
        // D s_26_76: cast reint s_26_75 -> u20
        let s_26_76: u32 = (s_26_75.value() as u32);
        // D s_26_77: write-var iss <= s_26_76
        fn_state.iss = s_26_76;
        // N s_26_78: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#30664 <= s_27_0
        fn_state.gs_30664 = s_27_0;
        // N s_27_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#30665 <= s_28_0
        fn_state.gs_30665 = s_28_0;
        // N s_28_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_29_0: read-var except:struct
        let s_29_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_29_1: write-var return_value <= s_29_0
        fn_state.return_value = s_29_0;
        // N s_29_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_30_0: read-var ec:i
        let s_30_0: i128 = fn_state.ec;
        // C s_30_1: const #3s : i
        let s_30_1: i128 = 3;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // N s_30_4: branch s_30_3 b32 b31
        if s_30_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_31_0: const #2u : u32
        let s_31_0: u32 = 2;
        // S s_31_1: call ExceptionSyndrome(s_31_0)
        let s_31_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_31_0,
        );
        // D s_31_2: write-var except <= s_31_1
        fn_state.except = s_31_1;
        // N s_31_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_32_0: read-var ec:i
        let s_32_0: i128 = fn_state.ec;
        // C s_32_1: const #4s : i
        let s_32_1: i128 = 4;
        // D s_32_2: cmp-eq s_32_0 s_32_1
        let s_32_2: bool = ((s_32_0) == (s_32_1));
        // D s_32_3: not s_32_2
        let s_32_3: bool = !s_32_2;
        // N s_32_4: branch s_32_3 b34 b33
        if s_32_3 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_33_0: const #3u : u32
        let s_33_0: u32 = 3;
        // S s_33_1: call ExceptionSyndrome(s_33_0)
        let s_33_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_33_0,
        );
        // D s_33_2: write-var except <= s_33_1
        fn_state.except = s_33_1;
        // N s_33_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_34_0: read-var ec:i
        let s_34_0: i128 = fn_state.ec;
        // C s_34_1: const #5s : i
        let s_34_1: i128 = 5;
        // D s_34_2: cmp-eq s_34_0 s_34_1
        let s_34_2: bool = ((s_34_0) == (s_34_1));
        // D s_34_3: not s_34_2
        let s_34_3: bool = !s_34_2;
        // N s_34_4: branch s_34_3 b36 b35
        if s_34_3 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_35_0: const #4u : u32
        let s_35_0: u32 = 4;
        // S s_35_1: call ExceptionSyndrome(s_35_0)
        let s_35_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_35_0,
        );
        // D s_35_2: write-var except <= s_35_1
        fn_state.except = s_35_1;
        // N s_35_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_36_0: read-var ec:i
        let s_36_0: i128 = fn_state.ec;
        // C s_36_1: const #6s : i
        let s_36_1: i128 = 6;
        // D s_36_2: cmp-eq s_36_0 s_36_1
        let s_36_2: bool = ((s_36_0) == (s_36_1));
        // D s_36_3: not s_36_2
        let s_36_3: bool = !s_36_2;
        // N s_36_4: branch s_36_3 b38 b37
        if s_36_3 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_37_0: const #5u : u32
        let s_37_0: u32 = 5;
        // S s_37_1: call ExceptionSyndrome(s_37_0)
        let s_37_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_37_0,
        );
        // D s_37_2: write-var except <= s_37_1
        fn_state.except = s_37_1;
        // N s_37_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_38_0: read-var ec:i
        let s_38_0: i128 = fn_state.ec;
        // C s_38_1: const #7s : i
        let s_38_1: i128 = 7;
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // D s_38_3: not s_38_2
        let s_38_3: bool = !s_38_2;
        // N s_38_4: branch s_38_3 b40 b39
        if s_38_3 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_39_0: const #7u : u32
        let s_39_0: u32 = 7;
        // S s_39_1: call ExceptionSyndrome(s_39_0)
        let s_39_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_39_0,
        );
        // D s_39_2: write-var except <= s_39_1
        fn_state.except = s_39_1;
        // N s_39_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_40_0: read-var ec:i
        let s_40_0: i128 = fn_state.ec;
        // C s_40_1: const #8s : i
        let s_40_1: i128 = 8;
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // D s_40_3: not s_40_2
        let s_40_3: bool = !s_40_2;
        // N s_40_4: branch s_40_3 b42 b41
        if s_40_3 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_41_0: const #8u : u32
        let s_41_0: u32 = 8;
        // S s_41_1: call ExceptionSyndrome(s_41_0)
        let s_41_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_41_0,
        );
        // D s_41_2: write-var except <= s_41_1
        fn_state.except = s_41_1;
        // N s_41_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_42_0: read-var ec:i
        let s_42_0: i128 = fn_state.ec;
        // C s_42_1: const #12s : i
        let s_42_1: i128 = 12;
        // D s_42_2: cmp-eq s_42_0 s_42_1
        let s_42_2: bool = ((s_42_0) == (s_42_1));
        // D s_42_3: not s_42_2
        let s_42_3: bool = !s_42_2;
        // N s_42_4: branch s_42_3 b44 b43
        if s_42_3 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_43_0: const #6u : u32
        let s_43_0: u32 = 6;
        // S s_43_1: call ExceptionSyndrome(s_43_0)
        let s_43_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_43_0,
        );
        // D s_43_2: write-var except <= s_43_1
        fn_state.except = s_43_1;
        // N s_43_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call Unreachable(s_44_0)
        let s_44_1: () = Unreachable(state, tracer, s_44_0);
        // N s_44_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
