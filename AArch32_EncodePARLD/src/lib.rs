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
use u_update_PAR_Type_FST::*;
use PAREncodeShareability::*;
use ReportedPARAttrs::*;
use u_update_PAR_Type_PA::*;
use PAR_read::*;
use u__UNKNOWN_bit::*;
use u_update_PAR_Type_S2WLK::*;
use PAR_write::*;
use u_update_PAR_Type_LPAE::*;
use ReportedPARShareability::*;
use u_update_PAR_Type_SH::*;
use Mk_PAR_Type::*;
use EncodePARAttrs::*;
use u_update_PAR_Type_ATTR::*;
use AArch32_PARFaultStatusLD::*;
use u_update_PAR_Type_FSTAGE::*;
use u__IMPDEF_bits::*;
use IsFault::*;
use u_update_PAR_Type_NS::*;
use u_update_PAR_Type_F::*;
use u__IMPDEF_bit::*;
use common::*;
pub fn AArch32_EncodePARLD<T: Tracer>(
    state: &mut State,
    tracer: &T,
    addrdesc: ProductTypece7c66ccb2cab13e,
    ss: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_23604: ProductType5c790c8ef59cc8b2,
        ga_23588: ProductTypeda0231e9dc169f81,
        ga_23605: bool,
        nsshadow_561: bool,
        ga_23600: bool,
        ga_23602: ProductType1d757adad216cdef,
        ga_23559: ProductTypeda0231e9dc169f81,
        ga_23571: ProductType5c790c8ef59cc8b2,
        ga_23578: ProductTypeda0231e9dc169f81,
        ga_23609: ProductType5c790c8ef59cc8b2,
        ga_23597: ProductType1d757adad216cdef,
        ga_23599: ProductType5c790c8ef59cc8b2,
        addrdesc: ProductTypece7c66ccb2cab13e,
        ss: u32,
    }
    let fn_state = FunctionState {
        addrdesc,
        ss,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var addrdesc:struct
        let s_0_0: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_0_1: call IsFault(s_0_0)
        let s_0_1: bool = IsFault(state, tracer, s_0_0);
        // D s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b8 b1
        if s_0_2 {
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call PAR_read(s_1_0)
        let s_1_1: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_1_0);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // S s_1_3: call _update_PAR_Type_F(s_1_1, s_1_2)
        let s_1_3: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_F(
            state,
            tracer,
            s_1_1,
            s_1_2,
        );
        // S s_1_4: call PAR_write(s_1_3)
        let s_1_4: () = PAR_write(state, tracer, s_1_3);
        // C s_1_5: const #() : ()
        let s_1_5: () = ();
        // S s_1_6: call PAR_read(s_1_5)
        let s_1_6: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_1_5);
        // D s_1_7: read-var addrdesc.0:struct
        let s_1_7: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_1_8: call AArch32_PARFaultStatusLD(s_1_7)
        let s_1_8: u8 = AArch32_PARFaultStatusLD(state, tracer, s_1_7);
        // D s_1_9: call _update_PAR_Type_FST(s_1_6, s_1_8)
        let s_1_9: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_FST(
            state,
            tracer,
            s_1_6,
            s_1_8,
        );
        // D s_1_10: call PAR_write(s_1_9)
        let s_1_10: () = PAR_write(state, tracer, s_1_9);
        // C s_1_11: const #() : ()
        let s_1_11: () = ();
        // S s_1_12: call PAR_read(s_1_11)
        let s_1_12: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_1_11);
        // D s_1_13: write-var ga#23599 <= s_1_12
        fn_state.ga_23599 = s_1_12;
        // D s_1_14: read-var addrdesc.0:struct
        let s_1_14: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_1_15: write-var ga#23597 <= s_1_14
        fn_state.ga_23597 = s_1_14;
        // D s_1_16: read-var ga#23597.14:struct
        let s_1_16: bool = fn_state.ga_23597._14;
        // N s_1_17: branch s_1_16 b7 b2
        if s_1_16 {
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
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var ga#23600 <= s_2_0
        fn_state.ga_23600 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#23599:struct
        let s_3_0: ProductType5c790c8ef59cc8b2 = fn_state.ga_23599;
        // D s_3_1: read-var ga#23600:u8
        let s_3_1: bool = fn_state.ga_23600;
        // D s_3_2: call _update_PAR_Type_S2WLK(s_3_0, s_3_1)
        let s_3_2: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_S2WLK(
            state,
            tracer,
            s_3_0,
            s_3_1,
        );
        // D s_3_3: call PAR_write(s_3_2)
        let s_3_3: () = PAR_write(state, tracer, s_3_2);
        // C s_3_4: const #() : ()
        let s_3_4: () = ();
        // S s_3_5: call PAR_read(s_3_4)
        let s_3_5: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_3_4);
        // D s_3_6: write-var ga#23604 <= s_3_5
        fn_state.ga_23604 = s_3_5;
        // D s_3_7: read-var addrdesc.0:struct
        let s_3_7: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_3_8: write-var ga#23602 <= s_3_7
        fn_state.ga_23602 = s_3_7;
        // D s_3_9: read-var ga#23602.15:struct
        let s_3_9: bool = fn_state.ga_23602._15;
        // N s_3_10: branch s_3_9 b6 b4
        if s_3_9 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var ga#23605 <= s_4_0
        fn_state.ga_23605 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#23604:struct
        let s_5_0: ProductType5c790c8ef59cc8b2 = fn_state.ga_23604;
        // D s_5_1: read-var ga#23605:u8
        let s_5_1: bool = fn_state.ga_23605;
        // D s_5_2: call _update_PAR_Type_FSTAGE(s_5_0, s_5_1)
        let s_5_2: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_FSTAGE(
            state,
            tracer,
            s_5_0,
            s_5_1,
        );
        // D s_5_3: call PAR_write(s_5_2)
        let s_5_3: () = PAR_write(state, tracer, s_5_2);
        // C s_5_4: const #() : ()
        let s_5_4: () = ();
        // S s_5_5: call PAR_read(s_5_4)
        let s_5_5: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_5_4);
        // C s_5_6: const #1u : u8
        let s_5_6: bool = true;
        // S s_5_7: call _update_PAR_Type_LPAE(s_5_5, s_5_6)
        let s_5_7: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_LPAE(
            state,
            tracer,
            s_5_5,
            s_5_6,
        );
        // S s_5_8: call PAR_write(s_5_7)
        let s_5_8: () = PAR_write(state, tracer, s_5_7);
        // C s_5_9: const #() : ()
        let s_5_9: () = ();
        // S s_5_10: call PAR_read(s_5_9)
        let s_5_10: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_5_9);
        // D s_5_11: write-var ga#23609 <= s_5_10
        fn_state.ga_23609 = s_5_10;
        // D s_5_12: read-var ga#23609.0:struct
        let s_5_12: u64 = fn_state.ga_23609._0;
        // C s_5_13: const #16s : i64
        let s_5_13: i64 = 16;
        // C s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // C s_5_15: const #"Faulting PAR" : str
        let s_5_15: &'static str = "Faulting PAR";
        // S s_5_16: call __IMPDEF_bits(s_5_14, s_5_15)
        let s_5_16: Bits = u__IMPDEF_bits(state, tracer, s_5_14, s_5_15);
        // S s_5_17: cast reint s_5_16 -> u16
        let s_5_17: u16 = (s_5_16.value() as u16);
        // C s_5_18: const #48s : i
        let s_5_18: i128 = 48;
        // D s_5_19: cast zx s_5_12 -> bv
        let s_5_19: Bits = Bits::new(s_5_12 as u128, 64u16);
        // S s_5_20: cast zx s_5_17 -> bv
        let s_5_20: Bits = Bits::new(s_5_17 as u128, 16u16);
        // C s_5_21: const #15s : i
        let s_5_21: i128 = 15;
        // C s_5_22: const #1u : u64
        let s_5_22: u64 = 1;
        // C s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 64u16);
        // C s_5_24: lsl s_5_23 s_5_21
        let s_5_24: Bits = s_5_23 << s_5_21;
        // C s_5_25: sub s_5_24 s_5_23
        let s_5_25: Bits = ((s_5_24) - (s_5_23));
        // S s_5_26: and s_5_20 s_5_25
        let s_5_26: Bits = ((s_5_20) & (s_5_25));
        // S s_5_27: lsl s_5_26 s_5_18
        let s_5_27: Bits = s_5_26 << s_5_18;
        // C s_5_28: lsl s_5_25 s_5_18
        let s_5_28: Bits = s_5_25 << s_5_18;
        // C s_5_29: cmpl s_5_28
        let s_5_29: Bits = !s_5_28;
        // D s_5_30: and s_5_19 s_5_29
        let s_5_30: Bits = ((s_5_19) & (s_5_29));
        // D s_5_31: or s_5_30 s_5_27
        let s_5_31: Bits = ((s_5_30) | (s_5_27));
        // D s_5_32: cast reint s_5_31 -> u64
        let s_5_32: u64 = (s_5_31.value() as u64);
        // D s_5_33: call Mk_PAR_Type(s_5_32)
        let s_5_33: ProductType5c790c8ef59cc8b2 = Mk_PAR_Type(state, tracer, s_5_32);
        // D s_5_34: call PAR_write(s_5_33)
        let s_5_34: () = PAR_write(state, tracer, s_5_33);
        // N s_5_35: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var ga#23605 <= s_6_0
        fn_state.ga_23605 = s_6_0;
        // N s_6_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var ga#23600 <= s_7_0
        fn_state.ga_23600 = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ss:u32
        let s_8_0: u32 = fn_state.ss;
        // C s_8_1: const #0u : u32
        let s_8_1: u32 = 0;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b16 b9
        if s_8_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var addrdesc.3:struct
        let s_9_0: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_9_1: write-var ga#23559 <= s_9_0
        fn_state.ga_23559 = s_9_0;
        // D s_9_2: read-var ga#23559.1:struct
        let s_9_2: u32 = fn_state.ga_23559._1;
        // C s_9_3: const #1u : u32
        let s_9_3: u32 = 1;
        // D s_9_4: cmp-eq s_9_2 s_9_3
        let s_9_4: bool = ((s_9_2) == (s_9_3));
        // N s_9_5: branch s_9_4 b15 b10
        if s_9_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var addrdesc.3:struct
        let s_11_0: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_11_1: write-var ga#23588 <= s_11_0
        fn_state.ga_23588 = s_11_0;
        // D s_11_2: read-var ga#23588.1:struct
        let s_11_2: u32 = fn_state.ga_23588._1;
        // C s_11_3: const #1u : u32
        let s_11_3: u32 = 1;
        // D s_11_4: cmp-eq s_11_2 s_11_3
        let s_11_4: bool = ((s_11_2) == (s_11_3));
        // N s_11_5: branch s_11_4 b14 b12
        if s_11_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var nsshadow#561 <= s_12_0
        fn_state.nsshadow_561 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call PAR_read(s_13_0)
        let s_13_1: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_13_0);
        // C s_13_2: const #0u : u8
        let s_13_2: bool = false;
        // S s_13_3: call _update_PAR_Type_F(s_13_1, s_13_2)
        let s_13_3: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_F(
            state,
            tracer,
            s_13_1,
            s_13_2,
        );
        // S s_13_4: call PAR_write(s_13_3)
        let s_13_4: () = PAR_write(state, tracer, s_13_3);
        // C s_13_5: const #() : ()
        let s_13_5: () = ();
        // S s_13_6: call PAR_read(s_13_5)
        let s_13_6: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_13_5);
        // D s_13_7: read-var addrdesc.2:struct
        let s_13_7: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_13_8: call PAREncodeShareability(s_13_7)
        let s_13_8: u8 = PAREncodeShareability(state, tracer, s_13_7);
        // D s_13_9: call ReportedPARShareability(s_13_8)
        let s_13_9: u8 = ReportedPARShareability(state, tracer, s_13_8);
        // D s_13_10: call _update_PAR_Type_SH(s_13_6, s_13_9)
        let s_13_10: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_SH(
            state,
            tracer,
            s_13_6,
            s_13_9,
        );
        // D s_13_11: call PAR_write(s_13_10)
        let s_13_11: () = PAR_write(state, tracer, s_13_10);
        // C s_13_12: const #() : ()
        let s_13_12: () = ();
        // S s_13_13: call PAR_read(s_13_12)
        let s_13_13: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_13_12);
        // D s_13_14: read-var nsshadow#561:u8
        let s_13_14: bool = fn_state.nsshadow_561;
        // D s_13_15: call _update_PAR_Type_NS(s_13_13, s_13_14)
        let s_13_15: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_NS(
            state,
            tracer,
            s_13_13,
            s_13_14,
        );
        // D s_13_16: call PAR_write(s_13_15)
        let s_13_16: () = PAR_write(state, tracer, s_13_15);
        // C s_13_17: const #() : ()
        let s_13_17: () = ();
        // S s_13_18: call PAR_read(s_13_17)
        let s_13_18: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_13_17);
        // D s_13_19: write-var ga#23571 <= s_13_18
        fn_state.ga_23571 = s_13_18;
        // D s_13_20: read-var ga#23571.0:struct
        let s_13_20: u64 = fn_state.ga_23571._0;
        // C s_13_21: const #"Non-Faulting PAR" : str
        let s_13_21: &'static str = "Non-Faulting PAR";
        // S s_13_22: call __IMPDEF_bit(s_13_21)
        let s_13_22: bool = u__IMPDEF_bit(state, tracer, s_13_21);
        // C s_13_23: const #10s : i
        let s_13_23: i128 = 10;
        // D s_13_24: cast zx s_13_20 -> bv
        let s_13_24: Bits = Bits::new(s_13_20 as u128, 64u16);
        // S s_13_25: cast zx s_13_22 -> bv
        let s_13_25: Bits = Bits::new(s_13_22 as u128, 1u16);
        // C s_13_26: const #0s : i
        let s_13_26: i128 = 0;
        // C s_13_27: const #1u : u64
        let s_13_27: u64 = 1;
        // C s_13_28: cast zx s_13_27 -> bv
        let s_13_28: Bits = Bits::new(s_13_27 as u128, 64u16);
        // C s_13_29: lsl s_13_28 s_13_26
        let s_13_29: Bits = s_13_28 << s_13_26;
        // C s_13_30: sub s_13_29 s_13_28
        let s_13_30: Bits = ((s_13_29) - (s_13_28));
        // S s_13_31: and s_13_25 s_13_30
        let s_13_31: Bits = ((s_13_25) & (s_13_30));
        // S s_13_32: lsl s_13_31 s_13_23
        let s_13_32: Bits = s_13_31 << s_13_23;
        // C s_13_33: lsl s_13_30 s_13_23
        let s_13_33: Bits = s_13_30 << s_13_23;
        // C s_13_34: cmpl s_13_33
        let s_13_34: Bits = !s_13_33;
        // D s_13_35: and s_13_24 s_13_34
        let s_13_35: Bits = ((s_13_24) & (s_13_34));
        // D s_13_36: or s_13_35 s_13_32
        let s_13_36: Bits = ((s_13_35) | (s_13_32));
        // D s_13_37: cast reint s_13_36 -> u64
        let s_13_37: u64 = (s_13_36.value() as u64);
        // D s_13_38: call Mk_PAR_Type(s_13_37)
        let s_13_38: ProductType5c790c8ef59cc8b2 = Mk_PAR_Type(state, tracer, s_13_37);
        // D s_13_39: call PAR_write(s_13_38)
        let s_13_39: () = PAR_write(state, tracer, s_13_38);
        // C s_13_40: const #() : ()
        let s_13_40: () = ();
        // S s_13_41: call PAR_read(s_13_40)
        let s_13_41: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_13_40);
        // C s_13_42: const #1u : u8
        let s_13_42: bool = true;
        // S s_13_43: call _update_PAR_Type_LPAE(s_13_41, s_13_42)
        let s_13_43: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_LPAE(
            state,
            tracer,
            s_13_41,
            s_13_42,
        );
        // S s_13_44: call PAR_write(s_13_43)
        let s_13_44: () = PAR_write(state, tracer, s_13_43);
        // C s_13_45: const #() : ()
        let s_13_45: () = ();
        // S s_13_46: call PAR_read(s_13_45)
        let s_13_46: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_13_45);
        // D s_13_47: read-var addrdesc.3:struct
        let s_13_47: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_13_48: write-var ga#23578 <= s_13_47
        fn_state.ga_23578 = s_13_47;
        // D s_13_49: read-var ga#23578.0:struct
        let s_13_49: u64 = fn_state.ga_23578._0;
        // C s_13_50: const #12s : i
        let s_13_50: i128 = 12;
        // D s_13_51: cast zx s_13_49 -> bv
        let s_13_51: Bits = Bits::new(s_13_49 as u128, 56u16);
        // C s_13_52: const #1s : i64
        let s_13_52: i64 = 1;
        // C s_13_53: cast zx s_13_52 -> i
        let s_13_53: i128 = (i128::try_from(s_13_52).unwrap());
        // C s_13_54: const #27s : i
        let s_13_54: i128 = 27;
        // C s_13_55: add s_13_54 s_13_53
        let s_13_55: i128 = (s_13_54 + s_13_53);
        // D s_13_56: bit-extract s_13_51 s_13_50 s_13_55
        let s_13_56: Bits = (Bits::new(
            ((s_13_51) >> (s_13_50)).value(),
            u16::try_from(s_13_55).unwrap(),
        ));
        // D s_13_57: cast reint s_13_56 -> u28
        let s_13_57: u32 = (s_13_56.value() as u32);
        // D s_13_58: call _update_PAR_Type_PA(s_13_46, s_13_57)
        let s_13_58: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_PA(
            state,
            tracer,
            s_13_46,
            s_13_57,
        );
        // D s_13_59: call PAR_write(s_13_58)
        let s_13_59: () = PAR_write(state, tracer, s_13_58);
        // C s_13_60: const #() : ()
        let s_13_60: () = ();
        // S s_13_61: call PAR_read(s_13_60)
        let s_13_61: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_13_60);
        // D s_13_62: read-var addrdesc.2:struct
        let s_13_62: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_13_63: call EncodePARAttrs(s_13_62)
        let s_13_63: u8 = EncodePARAttrs(state, tracer, s_13_62);
        // D s_13_64: call ReportedPARAttrs(s_13_63)
        let s_13_64: u8 = ReportedPARAttrs(state, tracer, s_13_63);
        // D s_13_65: call _update_PAR_Type_ATTR(s_13_61, s_13_64)
        let s_13_65: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_ATTR(
            state,
            tracer,
            s_13_61,
            s_13_64,
        );
        // D s_13_66: call PAR_write(s_13_65)
        let s_13_66: () = PAR_write(state, tracer, s_13_65);
        // N s_13_67: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var nsshadow#561 <= s_14_0
        fn_state.nsshadow_561 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call __UNKNOWN_bit(s_16_0)
        let s_16_1: bool = u__UNKNOWN_bit(state, tracer, s_16_0);
        // N s_16_2: jump b11
        return block_11(state, tracer, fn_state);
    }
}
