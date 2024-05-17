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
use AArch64_EncodeAsyncErrorSyndrome::*;
use Bit::*;
use u__IMPDEF_bits::*;
use Zeros::*;
use AArch64_PEErrorState::*;
use GetPendingPhysicalSError::*;
use common::*;
pub fn AArch64_PhysicalSErrorSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    implicit_esb: bool,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        fault: ProductType1d757adad216cdef,
        ga_18871: bool,
        syndrome: u32,
        errorstate: u32,
        implicit_esb: bool,
    }
    let fn_state = FunctionState {
        implicit_esb,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #25s : i
        let s_0_0: i128 = 25;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u25
        let s_0_2: u32 = (s_0_1.value() as u32);
        // D s_0_3: write-var syndrome <= s_0_2
        fn_state.syndrome = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call GetPendingPhysicalSError(s_0_4)
        let s_0_5: ProductType1d757adad216cdef = GetPendingPhysicalSError(
            state,
            tracer,
            s_0_4,
        );
        // D s_0_6: write-var fault <= s_0_5
        fn_state.fault = s_0_5;
        // D s_0_7: read-var fault:struct
        let s_0_7: ProductType1d757adad216cdef = fn_state.fault;
        // D s_0_8: call AArch64_PEErrorState(s_0_7)
        let s_0_8: u32 = AArch64_PEErrorState(state, tracer, s_0_7);
        // D s_0_9: write-var errorstate <= s_0_8
        fn_state.errorstate = s_0_8;
        // D s_0_10: read-var errorstate:u32
        let s_0_10: u32 = fn_state.errorstate;
        // C s_0_11: const #5u : u32
        let s_0_11: u32 = 5;
        // D s_0_12: cmp-eq s_0_10 s_0_11
        let s_0_12: bool = ((s_0_10) == (s_0_11));
        // N s_0_13: branch s_0_12 b8 b1
        if s_0_12 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_1_0: read-var errorstate:u32
        let s_1_0: u32 = fn_state.errorstate;
        // C s_1_1: const #6u : u32
        let s_1_1: u32 = 6;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b7 b2
        if s_1_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // S s_2_1: call Bit(s_2_0)
        let s_2_1: bool = Bit(state, tracer, s_2_0);
        // C s_2_2: const #24s : i
        let s_2_2: i128 = 24;
        // D s_2_3: read-var syndrome:u25
        let s_2_3: u32 = fn_state.syndrome;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 25u16);
        // C s_2_5: const #1u : u64
        let s_2_5: u64 = 1;
        // D s_2_6: bit-insert s_2_4 s_2_4 s_2_2 s_2_5
        let s_2_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_2_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_2_4.length(),
            );
            (s_2_4 & mask) | (s_2_4 << s_2_2)
        };
        // D s_2_7: cast reint s_2_6 -> u25
        let s_2_7: u32 = (s_2_6.value() as u32);
        // D s_2_8: write-var syndrome <= s_2_7
        fn_state.syndrome = s_2_7;
        // D s_2_9: read-var implicit_esb:u8
        let s_2_9: bool = fn_state.implicit_esb;
        // N s_2_10: branch s_2_9 b6 b3
        if s_2_9 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var ga#18871 <= s_3_0
        fn_state.ga_18871 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_4_0: read-var ga#18871:u8
        let s_4_0: bool = fn_state.ga_18871;
        // D s_4_1: call Bit(s_4_0)
        let s_4_1: bool = Bit(state, tracer, s_4_0);
        // C s_4_2: const #13s : i
        let s_4_2: i128 = 13;
        // D s_4_3: read-var syndrome:u25
        let s_4_3: u32 = fn_state.syndrome;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 25u16);
        // C s_4_5: const #1u : u64
        let s_4_5: u64 = 1;
        // D s_4_6: bit-insert s_4_4 s_4_4 s_4_2 s_4_5
        let s_4_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_4_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_4_4.length(),
            );
            (s_4_4 & mask) | (s_4_4 << s_4_2)
        };
        // D s_4_7: cast reint s_4_6 -> u25
        let s_4_7: u32 = (s_4_6.value() as u32);
        // D s_4_8: write-var syndrome <= s_4_7
        fn_state.syndrome = s_4_7;
        // D s_4_9: read-var errorstate:u32
        let s_4_9: u32 = fn_state.errorstate;
        // D s_4_10: call AArch64_EncodeAsyncErrorSyndrome(s_4_9)
        let s_4_10: u8 = AArch64_EncodeAsyncErrorSyndrome(state, tracer, s_4_9);
        // C s_4_11: const #10s : i
        let s_4_11: i128 = 10;
        // D s_4_12: read-var syndrome:u25
        let s_4_12: u32 = fn_state.syndrome;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 25u16);
        // D s_4_14: cast zx s_4_10 -> bv
        let s_4_14: Bits = Bits::new(s_4_10 as u128, 3u16);
        // C s_4_15: const #2s : i
        let s_4_15: i128 = 2;
        // C s_4_16: const #1u : u64
        let s_4_16: u64 = 1;
        // C s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 64u16);
        // C s_4_18: lsl s_4_17 s_4_15
        let s_4_18: Bits = s_4_17 << s_4_15;
        // C s_4_19: sub s_4_18 s_4_17
        let s_4_19: Bits = ((s_4_18) - (s_4_17));
        // D s_4_20: and s_4_14 s_4_19
        let s_4_20: Bits = ((s_4_14) & (s_4_19));
        // D s_4_21: lsl s_4_20 s_4_11
        let s_4_21: Bits = s_4_20 << s_4_11;
        // C s_4_22: lsl s_4_19 s_4_11
        let s_4_22: Bits = s_4_19 << s_4_11;
        // C s_4_23: cmpl s_4_22
        let s_4_23: Bits = !s_4_22;
        // D s_4_24: and s_4_13 s_4_23
        let s_4_24: Bits = ((s_4_13) & (s_4_23));
        // D s_4_25: or s_4_24 s_4_21
        let s_4_25: Bits = ((s_4_24) | (s_4_21));
        // D s_4_26: cast reint s_4_25 -> u25
        let s_4_26: u32 = (s_4_25.value() as u32);
        // D s_4_27: write-var syndrome <= s_4_26
        fn_state.syndrome = s_4_26;
        // D s_4_28: read-var fault.5:struct
        let s_4_28: bool = fn_state.fault._5;
        // D s_4_29: call Bit(s_4_28)
        let s_4_29: bool = Bit(state, tracer, s_4_28);
        // C s_4_30: const #9s : i
        let s_4_30: i128 = 9;
        // D s_4_31: read-var syndrome:u25
        let s_4_31: u32 = fn_state.syndrome;
        // D s_4_32: cast zx s_4_31 -> bv
        let s_4_32: Bits = Bits::new(s_4_31 as u128, 25u16);
        // C s_4_33: const #1u : u64
        let s_4_33: u64 = 1;
        // D s_4_34: bit-insert s_4_32 s_4_32 s_4_30 s_4_33
        let s_4_34: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_4_33 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_4_32.length(),
            );
            (s_4_32 & mask) | (s_4_32 << s_4_30)
        };
        // D s_4_35: cast reint s_4_34 -> u25
        let s_4_35: u32 = (s_4_34.value() as u32);
        // D s_4_36: write-var syndrome <= s_4_35
        fn_state.syndrome = s_4_35;
        // C s_4_37: const #0s : i
        let s_4_37: i128 = 0;
        // D s_4_38: read-var syndrome:u25
        let s_4_38: u32 = fn_state.syndrome;
        // D s_4_39: cast zx s_4_38 -> bv
        let s_4_39: Bits = Bits::new(s_4_38 as u128, 25u16);
        // C s_4_40: const #17u : u8
        let s_4_40: u8 = 17;
        // C s_4_41: cast zx s_4_40 -> bv
        let s_4_41: Bits = Bits::new(s_4_40 as u128, 6u16);
        // C s_4_42: const #5s : i
        let s_4_42: i128 = 5;
        // C s_4_43: const #1u : u64
        let s_4_43: u64 = 1;
        // C s_4_44: cast zx s_4_43 -> bv
        let s_4_44: Bits = Bits::new(s_4_43 as u128, 64u16);
        // C s_4_45: lsl s_4_44 s_4_42
        let s_4_45: Bits = s_4_44 << s_4_42;
        // C s_4_46: sub s_4_45 s_4_44
        let s_4_46: Bits = ((s_4_45) - (s_4_44));
        // C s_4_47: and s_4_41 s_4_46
        let s_4_47: Bits = ((s_4_41) & (s_4_46));
        // C s_4_48: lsl s_4_47 s_4_37
        let s_4_48: Bits = s_4_47 << s_4_37;
        // C s_4_49: lsl s_4_46 s_4_37
        let s_4_49: Bits = s_4_46 << s_4_37;
        // C s_4_50: cmpl s_4_49
        let s_4_50: Bits = !s_4_49;
        // D s_4_51: and s_4_39 s_4_50
        let s_4_51: Bits = ((s_4_39) & (s_4_50));
        // D s_4_52: or s_4_51 s_4_48
        let s_4_52: Bits = ((s_4_51) | (s_4_48));
        // D s_4_53: cast reint s_4_52 -> u25
        let s_4_53: u32 = (s_4_52.value() as u32);
        // D s_4_54: write-var syndrome <= s_4_53
        fn_state.syndrome = s_4_53;
        // N s_4_55: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_5_0: read-var syndrome:u25
        let s_5_0: u32 = fn_state.syndrome;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var ga#18871 <= s_6_0
        fn_state.ga_18871 = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // S s_7_1: call Bit(s_7_0)
        let s_7_1: bool = Bit(state, tracer, s_7_0);
        // C s_7_2: const #24s : i
        let s_7_2: i128 = 24;
        // D s_7_3: read-var syndrome:u25
        let s_7_3: u32 = fn_state.syndrome;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 25u16);
        // C s_7_5: const #1u : u64
        let s_7_5: u64 = 1;
        // D s_7_6: bit-insert s_7_4 s_7_4 s_7_2 s_7_5
        let s_7_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_7_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_7_4.length(),
            );
            (s_7_4 & mask) | (s_7_4 << s_7_2)
        };
        // D s_7_7: cast reint s_7_6 -> u25
        let s_7_7: u32 = (s_7_6.value() as u32);
        // D s_7_8: write-var syndrome <= s_7_7
        fn_state.syndrome = s_7_7;
        // C s_7_9: const #24s : i64
        let s_7_9: i64 = 24;
        // C s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // C s_7_11: const #"IMPDEF ErrorState" : str
        let s_7_11: &'static str = "IMPDEF ErrorState";
        // S s_7_12: call __IMPDEF_bits(s_7_10, s_7_11)
        let s_7_12: Bits = u__IMPDEF_bits(state, tracer, s_7_10, s_7_11);
        // S s_7_13: cast reint s_7_12 -> u24
        let s_7_13: u32 = (s_7_12.value() as u32);
        // C s_7_14: const #0s : i
        let s_7_14: i128 = 0;
        // D s_7_15: read-var syndrome:u25
        let s_7_15: u32 = fn_state.syndrome;
        // D s_7_16: cast zx s_7_15 -> bv
        let s_7_16: Bits = Bits::new(s_7_15 as u128, 25u16);
        // S s_7_17: cast zx s_7_13 -> bv
        let s_7_17: Bits = Bits::new(s_7_13 as u128, 24u16);
        // C s_7_18: const #23s : i
        let s_7_18: i128 = 23;
        // C s_7_19: const #1u : u64
        let s_7_19: u64 = 1;
        // C s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 64u16);
        // C s_7_21: lsl s_7_20 s_7_18
        let s_7_21: Bits = s_7_20 << s_7_18;
        // C s_7_22: sub s_7_21 s_7_20
        let s_7_22: Bits = ((s_7_21) - (s_7_20));
        // S s_7_23: and s_7_17 s_7_22
        let s_7_23: Bits = ((s_7_17) & (s_7_22));
        // S s_7_24: lsl s_7_23 s_7_14
        let s_7_24: Bits = s_7_23 << s_7_14;
        // C s_7_25: lsl s_7_22 s_7_14
        let s_7_25: Bits = s_7_22 << s_7_14;
        // C s_7_26: cmpl s_7_25
        let s_7_26: Bits = !s_7_25;
        // D s_7_27: and s_7_16 s_7_26
        let s_7_27: Bits = ((s_7_16) & (s_7_26));
        // D s_7_28: or s_7_27 s_7_24
        let s_7_28: Bits = ((s_7_27) | (s_7_24));
        // D s_7_29: cast reint s_7_28 -> u25
        let s_7_29: u32 = (s_7_28.value() as u32);
        // D s_7_30: write-var syndrome <= s_7_29
        fn_state.syndrome = s_7_29;
        // N s_7_31: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_8_0: const #25s : i
        let s_8_0: i128 = 25;
        // S s_8_1: call Zeros(s_8_0)
        let s_8_1: Bits = Zeros(state, tracer, s_8_0);
        // S s_8_2: cast reint s_8_1 -> u25
        let s_8_2: u32 = (s_8_1.value() as u32);
        // D s_8_3: write-var syndrome <= s_8_2
        fn_state.syndrome = s_8_2;
        // N s_8_4: jump b5
        return block_5(state, tracer, fn_state);
    }
}
