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
use AArch32_CommonFaultStatus::*;
use AArch32_EncodeAsyncErrorSyndrome::*;
use TTBCR_read::*;
use GetPendingPhysicalSError::*;
use AArch32_PEErrorState::*;
use Bit::*;
use u_get_TTBCR_Type_EAE::*;
use Zeros::*;
use common::*;
pub fn AArch32_PhysicalSErrorSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_24216: (),
) -> u16 {
    #[derive(Default)]
    struct FunctionState {
        fault: ProductType1d757adad216cdef,
        syndrome: u32,
        gs_24216: (),
    }
    let fn_state = FunctionState {
        gs_24216,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_0_0: const #32s : i
        let s_0_0: i128 = 32;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u32
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
        // C s_0_7: const #16975u : u32
        let s_0_7: u32 = 16975;
        // D s_0_8: read-reg s_0_7:u8
        let s_0_8: u8 = {
            let value = state.read_register::<u8>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 2u16);
        // C s_0_10: const #432u : u32
        let s_0_10: u32 = 432;
        // D s_0_11: read-reg s_0_10:u8
        let s_0_11: u8 = {
            let value = state.read_register::<u8>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // D s_0_13: cmp-eq s_0_9 s_0_12
        let s_0_13: bool = ((s_0_9) == (s_0_12));
        // N s_0_14: branch s_0_13 b3 b1
        if s_0_13 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call TTBCR_read(s_1_0)
        let s_1_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_1_0);
        // S s_1_2: call _get_TTBCR_Type_EAE(s_1_1)
        let s_1_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_1_1);
        // S s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // C s_1_4: const #1u : u8
        let s_1_4: bool = true;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // S s_1_6: cmp-eq s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) == (s_1_5));
        // D s_1_7: read-var fault:struct
        let s_1_7: ProductType1d757adad216cdef = fn_state.fault;
        // D s_1_8: call AArch32_CommonFaultStatus(s_1_7, s_1_6)
        let s_1_8: u32 = AArch32_CommonFaultStatus(state, tracer, s_1_7, s_1_6);
        // D s_1_9: write-var syndrome <= s_1_8
        fn_state.syndrome = s_1_8;
        // N s_1_10: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var syndrome:u32
        let s_2_1: u32 = fn_state.syndrome;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 32u16);
        // C s_2_3: const #1s : i64
        let s_2_3: i64 = 1;
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // C s_2_5: const #15s : i
        let s_2_5: i128 = 15;
        // C s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: bit-extract s_2_2 s_2_0 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u16
        let s_2_8: u16 = (s_2_7.value() as u16);
        // N s_2_9: return s_2_8
        return s_2_8;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_3_0: read-var fault:struct
        let s_3_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_3_1: call AArch32_PEErrorState(s_3_0)
        let s_3_1: u32 = AArch32_PEErrorState(state, tracer, s_3_0);
        // D s_3_2: call AArch32_EncodeAsyncErrorSyndrome(s_3_1)
        let s_3_2: u8 = AArch32_EncodeAsyncErrorSyndrome(state, tracer, s_3_1);
        // C s_3_3: const #10s : i
        let s_3_3: i128 = 10;
        // D s_3_4: read-var syndrome:u32
        let s_3_4: u32 = fn_state.syndrome;
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 32u16);
        // D s_3_6: cast zx s_3_2 -> bv
        let s_3_6: Bits = Bits::new(s_3_2 as u128, 2u16);
        // C s_3_7: const #1s : i
        let s_3_7: i128 = 1;
        // C s_3_8: const #1u : u64
        let s_3_8: u64 = 1;
        // C s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 64u16);
        // C s_3_10: lsl s_3_9 s_3_7
        let s_3_10: Bits = s_3_9 << s_3_7;
        // C s_3_11: sub s_3_10 s_3_9
        let s_3_11: Bits = ((s_3_10) - (s_3_9));
        // D s_3_12: and s_3_6 s_3_11
        let s_3_12: Bits = ((s_3_6) & (s_3_11));
        // D s_3_13: lsl s_3_12 s_3_3
        let s_3_13: Bits = s_3_12 << s_3_3;
        // C s_3_14: lsl s_3_11 s_3_3
        let s_3_14: Bits = s_3_11 << s_3_3;
        // C s_3_15: cmpl s_3_14
        let s_3_15: Bits = !s_3_14;
        // D s_3_16: and s_3_5 s_3_15
        let s_3_16: Bits = ((s_3_5) & (s_3_15));
        // D s_3_17: or s_3_16 s_3_13
        let s_3_17: Bits = ((s_3_16) | (s_3_13));
        // D s_3_18: cast reint s_3_17 -> u32
        let s_3_18: u32 = (s_3_17.value() as u32);
        // D s_3_19: write-var syndrome <= s_3_18
        fn_state.syndrome = s_3_18;
        // D s_3_20: read-var fault.5:struct
        let s_3_20: bool = fn_state.fault._5;
        // D s_3_21: call Bit(s_3_20)
        let s_3_21: bool = Bit(state, tracer, s_3_20);
        // C s_3_22: const #9s : i
        let s_3_22: i128 = 9;
        // D s_3_23: read-var syndrome:u32
        let s_3_23: u32 = fn_state.syndrome;
        // D s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 32u16);
        // C s_3_25: const #1u : u64
        let s_3_25: u64 = 1;
        // D s_3_26: bit-insert s_3_24 s_3_24 s_3_22 s_3_25
        let s_3_26: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_3_25 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_3_24.length(),
            );
            (s_3_24 & mask) | (s_3_24 << s_3_22)
        };
        // D s_3_27: cast reint s_3_26 -> u32
        let s_3_27: u32 = (s_3_26.value() as u32);
        // D s_3_28: write-var syndrome <= s_3_27
        fn_state.syndrome = s_3_27;
        // C s_3_29: const #0s : i
        let s_3_29: i128 = 0;
        // D s_3_30: read-var syndrome:u32
        let s_3_30: u32 = fn_state.syndrome;
        // D s_3_31: cast zx s_3_30 -> bv
        let s_3_31: Bits = Bits::new(s_3_30 as u128, 32u16);
        // C s_3_32: const #17u : u8
        let s_3_32: u8 = 17;
        // C s_3_33: cast zx s_3_32 -> bv
        let s_3_33: Bits = Bits::new(s_3_32 as u128, 6u16);
        // C s_3_34: const #5s : i
        let s_3_34: i128 = 5;
        // C s_3_35: const #1u : u64
        let s_3_35: u64 = 1;
        // C s_3_36: cast zx s_3_35 -> bv
        let s_3_36: Bits = Bits::new(s_3_35 as u128, 64u16);
        // C s_3_37: lsl s_3_36 s_3_34
        let s_3_37: Bits = s_3_36 << s_3_34;
        // C s_3_38: sub s_3_37 s_3_36
        let s_3_38: Bits = ((s_3_37) - (s_3_36));
        // C s_3_39: and s_3_33 s_3_38
        let s_3_39: Bits = ((s_3_33) & (s_3_38));
        // C s_3_40: lsl s_3_39 s_3_29
        let s_3_40: Bits = s_3_39 << s_3_29;
        // C s_3_41: lsl s_3_38 s_3_29
        let s_3_41: Bits = s_3_38 << s_3_29;
        // C s_3_42: cmpl s_3_41
        let s_3_42: Bits = !s_3_41;
        // D s_3_43: and s_3_31 s_3_42
        let s_3_43: Bits = ((s_3_31) & (s_3_42));
        // D s_3_44: or s_3_43 s_3_40
        let s_3_44: Bits = ((s_3_43) | (s_3_40));
        // D s_3_45: cast reint s_3_44 -> u32
        let s_3_45: u32 = (s_3_44.value() as u32);
        // D s_3_46: write-var syndrome <= s_3_45
        fn_state.syndrome = s_3_45;
        // N s_3_47: jump b2
        return block_2(state, tracer, fn_state);
    }
}
